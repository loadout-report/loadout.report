import asyncio
from requests import get
import urllib.parse
from functools import cache

API_KEY = "ab0d8e6592644bae8134fd1fb7a27183"
headers = {
    'x-api-key': API_KEY
}


def get_manifest():
    return get(f"https://www.bungie.net/")

@cache
def get_definitions():
    return get(f"https://api.destinytrialsreport.com/destiny2/en/DestinyDefinitions.json").json()


@cache
def search_player(name):
    return get(f"https://elastic.destinytrialsreport.com/players/0/{urllib.parse.quote(name)}/").json()


@cache
def get_profile_information(membership_type, membership_id):
    response = get(f"https://www.bungie.net/Platform/Destiny2/{membership_type}/Profile/{membership_id}/",
                   params={'components': '100,104,200,202,205,300,305,306,800,900,1100'}, headers=headers)
    return response.json()[
        'Response']


def get_last_played(characters):
    latest = None
    for char_id, character in characters.items():
        if latest is None or character['dateLastPlayed'] > latest['dateLastPlayed']:
            latest = character
    return latest


def lookup_hash(hash):
    definitions = get_definitions()
    item = definitions['items'][str(hash)]
    return item


def main():
    print_player_inventory()
    # definitions = get_definitions()


def print_player_inventory():
    player_name = "Raptor#0751"
    result = search_player(player_name)
    player = result[0]  # assume first result is accurate
    membership_id = player['membershipId']
    print(membership_id)
    membership_type = "3"
    profile = get_profile_information(membership_type, membership_id)
    characters = profile['characters']['data']
    last_played = get_last_played(characters)
    equipment = profile['characterEquipment']['data'][last_played['characterId']]['items']
    print(len(equipment))
    item_components = profile['itemComponents']
    for item in equipment:
        item_hash = item['itemHash']
        lookup = lookup_hash(item_hash)
        name = lookup['n']
        instance_id = item['itemInstanceId']
        itype = lookup['t']
        if itype not in ['Emote Collection', 'Clan Banner', 'Transmat Effect', 'Sparrow', 'Ship', 'Ghost Shell',
                         'Finisher Collection', 'Artifact', 'Emblem']:
            print('---', itype, item_hash, instance_id, name)
            if instance_id in item_components['sockets']['data']:
                item_sockets = item_components['sockets']['data'][instance_id]['sockets']
                for socket in item_sockets:
                    if 'plugHash' in socket:
                        plug = lookup_hash(socket['plugHash'])
                        plug_name = plug['n']
                        plug_type = plug['t']
                        if plug_type not in ['', 'Shader', 'Memento', 'Weapon Ornament', 'Armor Ornament',
                                             'Restore Defaults'] \
                                and plug_name not in ['Empty Mod Socket', 'Empty Memento Socket', 'Kill Tracker',
                                                      'Crucible Tracker',
                                                      'Empty Fragment Socket'] and 'Catalyst' not in plug_name:
                            print(plug_type, plug_name)


if __name__ == '__main__':
    main()
