

pub struct LostSectors {
    pub completed: LostSectorDifficulty,
    pub not_completed: LostSectorDifficulty,
}

pub struct LostSectorDifficulty {
    pub legend: LostSectorCounters,
    pub master: LostSectorCounters,
}

pub struct LostSectorCounters {
    aphelions_rest: u64,
    bay_of_drowned_wishes: u64,
    bunker_e15: u64,
    chamber_of_starlight: u64,
    concealed_void: u64,
    excavation_site_xii: u64,
    exodus_garden_2a: u64,
    extraction: u64,
    k1_communion: u64,
    k1_krew_quarters: u64,
    k1_logistics: u64,
    k1_revelation: u64,
    metamorphosis: u64,
    perdition: u64,
    scavengers_den: u64,
    sepulcher: u64,
    skydock_iv: u64,
    the_conflux: u64,
    the_quarry: u64,
    the_rift: u64,
    veles_labyrinth: u64,
}
