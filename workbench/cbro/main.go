package main

import (
	"bufio"
	"bytes"
	"encoding/json"
	"flag"
	"fmt"
	"io"
	"io/ioutil"
	"log"
	"os"
	"os/signal"
	"runtime/pprof"
	"runtime/trace"
	"strconv"
	"strings"
	"sync"
	"sync/atomic"
	"time"
	"unicode"

	jsongoccy "github.com/goccy/go-json"
	"github.com/klauspost/compress/zstd"
	"github.com/paulbellamy/ratecounter"
)

var (
	activityDefinitionPath = flag.String("activity_definition", "activityDefinition.json", "path to activityDefinition.json")
	cpuprofile             = flag.String("cpuprofile", "", "write cpu profile to file")
)

var (
	rateIntervalSeconds = 1
	rateInterval        = time.Second * time.Duration(rateIntervalSeconds)
	rateCounter         = ratecounter.NewRateCounter(rateInterval)
	readCounter         = ratecounter.NewRateCounter(rateInterval)
	reduceCounter       = ratecounter.NewRateCounter(rateInterval)
	bytesRead           int64
)

var activityDefinition map[string]struct {
	DisplayProperties struct {
		Description string
		Name        string
	}
}

func init() {
}

type MapReduce[K comparable, V any] struct {
	ReduceCh func(int) chan map[K]V
	Map      func(pgcr []*pgcr) map[K]V
	Reduce   func(K, V, V) V
}

type strikeSeason struct {
	slug       string
	season     int
	completion bool
}

func (v strikeSeason) MarshalText() ([]byte, error) {
	return []byte(v.String()), nil
}

func (v strikeSeason) String() string {
	completed := "attempts"
	if v.completion {
		completed = "completions"
	}
	return fmt.Sprintf("%s/season%d/%s", v.slug, v.season, completed)
}

var StrikeCountMapReduce = MapReduce[string, int]{
	ReduceCh: func(size int) chan map[string]int { return make(chan map[string]int, size) },
	Map: func(pgcrs []*pgcr) map[string]int {
		m := make(map[string]int)
		for _, p := range pgcrs {
			k := fmt.Sprintf("strike/%s/season/%d", p.strikeSlug(), p.season())
			if p.isGM() != GM {
				continue
			}
			attemptKey := k + "/attempt"
			m[attemptKey] = m[attemptKey] + 1
			if p.completed() {
				completionKey := k + "/completion"
				m[completionKey] = m[completionKey] + 1
			}
			for _, e := range p.Entries {
				playerKey := fmt.Sprintf("player/%d/%s/%s", e.Player.DestinyUserInfo.MembershipType, e.Player.DestinyUserInfo.MembershipID, k)
				attemptKey := playerKey + "/attempt"
				m[attemptKey] = m[attemptKey] + 1
				if e.Values.Completed > .5 {
					completionKey := playerKey + "/completion"
					m[completionKey] = m[completionKey] + 1
				}
			}
		}
		return m
	},
	Reduce: func(k string, a, b int) int {
		return a + b
	},
}

func main() {

	flag.Parse()

	b, err := os.ReadFile(*activityDefinitionPath)
	if err != nil {
		log.Fatal(err)
	}
	if err := json.Unmarshal(b, &activityDefinition); err != nil {
		log.Fatal(err)
	}
	log.SetFlags(0)
	start := time.Now()
	defer func() { log.Printf("processed in %s", time.Since(start)) }()

	if *cpuprofile != "" {
		c := make(chan os.Signal, 1)
		signal.Notify(c)

		f, err := os.Create(*cpuprofile)
		if err != nil {
			log.Fatal(err)
		}
		pprof.StartCPUProfile(f)
		defer pprof.StopCPUProfile()
		blockFile, _ := os.Create(*cpuprofile + ".block")
		defer pprof.Lookup("block").WriteTo(blockFile, 0)
		traceFile, _ := os.Create(*cpuprofile + ".trace")
		defer pprof.Lookup("block").WriteTo(blockFile, 0)

		trace.Start(traceFile)
		defer trace.Stop()
		go func() {
			log.Printf("got signal %v, writing trace", <-c)
			trace.Stop()
		}()
	}

	var processed int64
	go func() {
		for {
			time.Sleep(time.Second)
			itemRate := float64(rateCounter.Rate()) / float64(rateIntervalSeconds)
			dataRate := float64(readCounter.Rate()) / float64(rateIntervalSeconds) / 1000 / 1000
			log.Printf("rate: %.0f MB/s %.0f/s total: %d", dataRate, itemRate, atomic.LoadInt64(&processed))
		}
	}()

	mapReduce := StrikeCountMapReduce

	const (
		decoders  = 12
		batchSize = 20 * (2 << 20) // 20 MB
	)
	var (
		toDecoder  = make(chan []byte, decoders*2)
		buffers    = make(chan []byte, decoders*2)
		toReducers = mapReduce.ReduceCh(32)
		decoderWG  sync.WaitGroup
	)
	for i := 0; i < decoders*2; i++ {
		buffers <- make([]byte, 0, batchSize)
	}
	for i := 0; i < decoders; i++ {
		decoderWG.Add(1)
		go func() {
			defer decoderWG.Done()

			var block []*pgcr
			for bs := range toDecoder {
				decoder := jsongoccy.NewDecoder(bytes.NewReader(bs))
				for {
					var p pgcr
					if err := decoder.Decode(&p); err != nil {
						if err == io.EOF {
							break
						}
						log.Fatal(err)
					}
					block = append(block, &p)
				}
				toReducers <- mapReduce.Map(block)
				atomic.AddInt64(&processed, int64(len(block)))
				rateCounter.Incr(int64(len(block)))
				block = block[:0]

				buffers <- bs[:0]
			}
		}()
	}

	var fileWG sync.WaitGroup
	var files []string
	fnScanner := bufio.NewScanner(os.Stdin)
	for fnScanner.Scan() {
		fileWG.Add(1)
		fn := fnScanner.Text()
		files = append(files, fn)
		go func() {
			defer fileWG.Done()
			log.Printf("Reading %s", fn)
			// each file
			f, err := os.Open(fn)
			if err != nil {
				log.Fatal(err)
			}
			zr, err := zstd.NewReader(wrappedReader{f})
			if err != nil {
				log.Fatal(err)
			}
			sc := bufio.NewScanner(zr)
			sc.Buffer(make([]byte, 0, 32<<20), 32<<20)
			var bs []byte
			for sc.Scan() {
				if bs == nil {
					bs = <-buffers
				}
				bs = append(bs, sc.Bytes()...)
				if len(bs) > batchSize {
					toDecoder <- bs
					bs = nil
				}
			}
			if sc.Err() != nil {
				log.Fatal(sc.Err())
			}
			if len(bs) > 0 {
				toDecoder <- bs
			}
			log.Printf("DONE %s", fn)
		}()
	}
	if err := fnScanner.Err(); err != nil {
		log.Fatal(err)
	}

	data := make(map[string]int)
	var reducerWG sync.WaitGroup
	reducerWG.Add(1)
	go func() {
		defer reducerWG.Done()

		for values := range toReducers {
			for k, v := range values {
				data[k] = mapReduce.Reduce(k, data[k], v)
			}
		}

		log.Print("Reducer done")
	}()

	fileWG.Wait()
	close(toDecoder)
	decoderWG.Wait()
	close(toReducers)
	reducerWG.Wait()

	result := struct {
		Duration   string
		InputFiles []string
		Processed  int64
		BytesRead  int64
		Result     interface{}
	}{
		Duration:   time.Since(start).Truncate(time.Second).String(),
		InputFiles: files,
		Processed:  atomic.LoadInt64(&processed),
		BytesRead:  atomic.LoadInt64(&bytesRead),
		Result:     data,
	}

	out, err := json.MarshalIndent(result, "", "\t")
	if err != nil {
		log.Fatal("json.MarshalIndent", err)
	}
	outFile := fmt.Sprintf("out-%d.json", time.Now().Unix())
	if err := ioutil.WriteFile(outFile, out, 700); err != nil {
		log.Fatal(err)
	}
	log.Printf("Written to %v", outFile)
}

type pgcr struct {
	ActivityDetails struct {
		InstanceID  string // PGCR id
		Modes       []int
		ReferenceID int64
	}
	Entries []struct {
		CharacterID string
		Player      struct {
			DestinyUserInfo struct {
				MembershipID                string
				MembershipType              int
				BungieGlobalDisplayName     string
				BungieGlobalDisplayNameCode int
			}
			EmblemHash int64
		}
		Values struct {
			ActivityDurationSeconds basicValue
			Score                   basicValue
			TimePlayedSeconds       basicValue
			Completed               basicValue
			CompletionReason        basicValue
		}
	}
	Period string
}

type NightfallLevel int

func (n NightfallLevel) String() string {
	switch n {
	case GM:
		return "Grandmaster"
	case NotGM:
		return "Other Nightfall"
	}
	return "Unknown"
}

const (
	Unknown NightfallLevel = iota
	NotGM
	GM
)

func (p pgcr) since() time.Duration {
	since := time.Since(p.end())
	return since - (since % time.Second)
}

func (p pgcr) end() time.Time {
	start, _ := time.Parse(time.RFC3339, p.Period)
	return start.Add(time.Second * time.Duration(p.Entries[0].Values.ActivityDurationSeconds))
}

func (p pgcr) duration() string {
	s := time.Duration(p.Entries[0].Values.ActivityDurationSeconds) * time.Second
	hs := ""
	if h := s.Hours(); h != 0 {
		hs = fmt.Sprintf("%.0fh", h)
	}
	ms := ""
	if h := s.Minutes(); h != 0 {
		ms = fmt.Sprintf("%.0fm", h)
	}
	ss := ""
	if h := s.Seconds(); h != 0 {
		ss = fmt.Sprintf("%.0fs", h)
	}
	return hs + ms + ss
}

func (p pgcr) shouldSave() bool {
	return p.isGM() == GM
}

func (p pgcr) completed() bool {
	for _, p := range p.Entries {
		if p.Values.CompletionReason < 0.5 {
			return true
		}
	}
	return false
}

func (p pgcr) id() int64 {
	id, err := strconv.ParseInt(p.ActivityDetails.InstanceID, 10, 64)
	if err != nil {
		log.Printf("pgcr %s could not decode id: %v", p.ActivityDetails.InstanceID, err)
	}
	return id
}

func (p pgcr) season() int {
	id := p.id()
	if id > 6_110_577_052 && id < 6_369_174_187 {
		// Worthy
		return 10
	}
	if id > 6_652_421_602 && id < 7_132_690_261 {
		// Arrivals
		return 11
	}
	if id > 7_618_446_979 && id < 7_927_738_348 {
		// Hunt
		return 12
	}
	if id > 8_157_068_252 && id < 8_418_736_467 {
		// Chosen
		return 13
	}
	if id > 8_700_774_175 && id < 9_028_999_691 {
		// Splicer
		return 14
	}
	if id > 9_370_573_998 && id < 10_179_519_298 {
		// Lost
		return 15
	}
	if id > 10_549_012_758 && id < 10_800_796_510 {
		// Risen
		return 16
	}
	if id > 11_087_977_829 && id < 11_369_612_788 {
		// Haunted
		return 17
	}
	if id >= 11_734_575_369 {
		// Plunder
		return 18
	}
	return 0
}

func (p pgcr) isNightfall() bool {
	for mode, _ := range p.ActivityDetails.Modes {
		if mode == 46 {
			return true
		}
	}
	return false
}

func (p pgcr) players() []string {
	var ids []string
	for _, e := range p.Entries {
		if e.Values.CompletionReason > .5 {
			continue
		}
		ids = append(ids, fmt.Sprintf("%s#%d", e.Player.DestinyUserInfo.BungieGlobalDisplayName, e.Player.DestinyUserInfo.BungieGlobalDisplayNameCode))
	}
	return ids
}

func (p pgcr) isGM() NightfallLevel {
	activity, ok := activityDefinition[fmt.Sprintf("%d", p.ActivityDetails.ReferenceID)]
	if !ok {
		return Unknown
	}
	if strings.Contains(activity.DisplayProperties.Name, "Grandmaster") {
		return GM
	}
	return NotGM
}

func (p pgcr) strikeSlug() string {
	activity, ok := activityDefinition[fmt.Sprintf("%d", p.ActivityDetails.ReferenceID)]
	if !ok {
		return "unknown"
	}
	return strings.Map(func(r rune) rune {
		if unicode.IsLetter(r) {
			return unicode.ToLower(r)
		}
		if unicode.IsSpace(r) {
			return '-'
		}
		return -1
	}, activity.DisplayProperties.Description)
}

type basicValue float64

type wrappedReader struct {
	io.Reader
}

func (wr wrappedReader) Read(b []byte) (n int, err error) {
	n, err = wr.Reader.Read(b)
	readCounter.Incr(int64(n))
	atomic.AddInt64(&bytesRead, int64(n))
	return n, err
}
