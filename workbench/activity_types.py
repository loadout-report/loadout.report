
from requests import get


def main():
    activity_definitions = get('https://www.bungie.net/common/destiny2_content/json/en/DestinyActivityDefinition-b589e03b-e55e-4a10-a136-490af7bdeec0.json').json()
    print(len(activity_definitions))
    gms = list([activity_id for activity_id in activity_definitions if 'Grandmaster' in activity_definitions[activity_id]['displayProperties']['name']])
    print(len(gms))
    ids = list(map(int, gms))
    ids.sort()
    print(ids)
    descriptions = [activity_definitions[gm]['displayProperties']['description'] for gm in gms]
    descriptions.sort()
    print(descriptions)



if __name__ == '__main__':
    main()
