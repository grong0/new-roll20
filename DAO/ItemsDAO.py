import json
import datetime
from enum import StrEnum


class PackItem:
    name: str
    quantity: int
    special: bool

    def __init__(self, object: dict | str) -> None:
        self.special = False
        if type(object) == dict:
            if "special" in object.keys():
                self.name = object["special"]
                self.special = True
            else:
                self.name = object["item"]
            self.quantity = object["quantity"] if "quantity" in object.keys() else 1
            return
        elif type(object) == str:
            self.name = object
            self.quantity = 1
        else:
            print("was neither dict or str")


class ItemType(StrEnum):
    FIREARM = "firearm"
    ARROW = "arrow"
    AXE = "axe"
    ARMOR = "armor"
    CLUB = "club"
    BOLT = "bolt"
    DAGGER = "dagger"
    SWORD = "sword"
    POLEARM = "polearm"
    CROSSBOW = "crossbow"
    SPEAR = "spear"
    HAMMER = "hammer"
    BOW = "bow"
    MACE = "mace"
    NET = "net"
    STAFF = "staff"
    NEITHER = "item"

    def __init__(self, name) -> None:
        super().__init__()


class MundaneItem:
    name: str
    source: str
    page: int
    type: str
    rarity: str
    weight: int
    weapon_category: str
    age: str
    property: list[str]
    range: str
    reload: int
    dmg1: str
    dmg_type: str
    # firearm: bool
    weapon: bool
    ammo_type: str
    srd: bool
    basic_rules: bool
    value: int
    # arrow: bool
    pack_contents: list[PackItem]
    dmg2: str
    # axe: bool
    entries: list[dict[str, list[str] | str]] | str
    ac: int
    # armor: bool
    strength: int
    stealth: bool
    # club: bool
    # bolt: bool
    scf_type: str
    # item_type: ItemType
    dagger: bool
    sword: bool
    polearm: bool
    crossbow: bool
    spear: bool
    hammer: bool
    bow: bool
    mace: bool
    net: bool
    staff: bool

    def __init__(self, object: dict) -> None:
        # print(object)
        self.name = object["name"]
        self.source = object["source"]
        self.page = object["page"] if "page" in object.keys() else -1
        self.type = object["type"] if "type" in object.keys() else ""
        self.rarity = object["rarity"] if "rarity" in object.keys() else "none"
        self.weight = object["weight"] if "weight" in object.keys() else 0
        self.weapon_category = (
            object["weaponCategory"] if "weaponCategory" in object.keys() else ""
        )
        self.age = object["age"] if "age" in object.keys() else ""
        self.property = object["property"] if "property" in object.keys() else []
        self.range = object["range"] if "range" in object.keys() else ""
        self.reload = object["reload"] if "reload" in object.keys() else 0
        self.dmg1 = object["dmg1"] if "dmg1" in object.keys() else ""
        self.dmg_type = object["dmgType"] if "dmgType" in object.keys() else ""
        self.weapon = object["weapon"] if "weapon" in object.keys() else False
        self.ammo_type = object["ammoType"] if "ammoType" in object.keys() else ""
        self.srd = object["srd"] if "srd" in object.keys() else False
        self.basic_rules = (
            object["basicRules"] if "basicRules" in object.keys() else False
        )
        self.value = object["value"] if "value" in object.keys() else -1
        self.pack_contents = (
            [PackItem(item) for item in object["packContents"]]
            if "packContents" in object.keys()
            else []
        )
        self.dmg2 = object["dmg2"] if "dmg2" in object.keys() else ""
        self.entries = object["entries"] if "entries" in object.keys() else []
        self.ac = object["ac"] if "ac" in object.keys() else 0
        self.strength = object["strength"] if "strength" in object.keys() else 0
        self.stealth = object["stealth"] if "stealth" in object.keys() else False
        self.scf_type = object["scfType"] if "scfType" in object.keys() else ""
        self.firearm = object["firearm"] if "firearm" in object.keys() else False
        self.arrow = object["arrow"] if "arrow" in object.keys() else False
        self.axe = object["axe"] if "axe" in object.keys() else False
        self.armor = object["armor"] if "armor" in object.keys() else False
        self.club = object["club"] if "club" in object.keys() else False
        self.bolt = object["bolt"] if "bolt" in object.keys() else False
        self.dagger = object["dagger"] if "dagger" in object.keys() else False
        self.sword = object["sword"] if "sword" in object.keys() else False
        self.polearm = object["polearm"] if "polearm" in object.keys() else False
        self.crossbow = object["crossbow"] if "crossbow" in object.keys() else False
        self.spear = object["spear"] if "spear" in object.keys() else False
        self.hammer = object["hammer"] if "hammer" in object.keys() else False
        self.bow = object["bow"] if "bow" in object.keys() else False
        self.mace = object["mace"] if "mace" in object.keys() else False
        self.net = object["net"] if "net" in object.keys() else False
        self.staff = object["staff"] if "staff" in object.keys() else False
        # self.item_type = self.get_item_type(object)

    def get_item_type(self, object: dict) -> ItemType:
        types = [
            "firearm"
            "arrow"
            "axe"
            "armor"
            "club"
            "bolt"
            "dagger"
            "sword"
            "polearm"
            "crossbow"
            "spear"
            "hammer"
            "bow"
            "mace"
            "net"
            "staff"
        ]

        valid_types = []

        # impl 1
        # for type in types:
        #     if type in object.keys():
        #         valid_types.append(type)
        # impl 2
        for key in object.keys():
            for _ in range(0, len(types)):
                if types[0] == key:
                    valid_types.append(key)
                types.pop(0)
        if len(valid_types) > 1:
            print(f"there are more than one valid_types: {valid_types}")
        elif len(valid_types) == 0:
            return ItemType("item")
        return ItemType(valid_types[0])


class MagicItem:
    

    def __init__(self) -> None:
        pass


class Items:
    mundane_items: dict[str, MundaneItem]
    magic_items: dict[str, MagicItem]

    def __init__(self) -> None:
        mundane_f = open("../data/raw/items-base.json")
        magic_f = open("../data/raw/items.json")

        mundane_object = json.load(mundane_f)
        magic_object = json.load(magic_f)

        self.mundane_items = {}
        self.magic_items = {}

        for base_item in mundane_object["baseitem"]:
            self.mundane_items[base_item["name"]] = MundaneItem(base_item)

        for item in magic_object["item"]:
            self.magic_items[item["name"]] = MagicItem(item)

        mundane_f.close()
        magic_f.close()


if __name__ == "__main__":
    # times = []
    # count = 1000
    # for _ in range(0, count):
    #     start = datetime.datetime.now()
    #     items = Items()
    #     end = datetime.datetime.now()
    #     times.append((end - start).microseconds)
    #     del items

    # average_time = sum(times) / count

    # print(f"average time was {average_time} microseconds")

    items = Items()
    print(items.items_base)
    print(items.items)
