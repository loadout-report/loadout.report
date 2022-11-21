
from requests import get


def main():
    activity_definitions = get('https://www.bungie.net/common/destiny2_content/json/en/DestinyActivityDefinition-b589e03b-e55e-4a10-a136-490af7bdeec0.json').json()
    print(len(activity_definitions))
    gms = list([activity_id for activity_id in activity_definitions if 'Grandmaster' in activity_definitions[activity_id]['displayProperties']['name']])
    print(len(gms))
    print(list(map(int, gms)))
    print([activity_definitions[gm]['displayProperties']['name'] for gm in gms])



if __name__ == '__main__':
    main()
