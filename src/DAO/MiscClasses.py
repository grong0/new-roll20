from enum import StrEnum
from typing import Optional


class Speed:
    walk: int
    fly: Optional[int] = None
    swim: Optional[int] = None
    climb: Optional[int] = None

    def __init__(self, object: int | dict[str, int]) -> None:
        if object is int:
            self.walk = object
        elif object is dict:
            self.walk = object["walk"]
            self.fly = object["fly"] if "fly" in object.keys() else self.fly
            self.swim = object["swim"] if "swim" in object.keys() else self.swim
            self.climb = object["climb"] if "climb" in object.keys() else self.climb


class Choose:
    count: int
    abilities: list[str]

    def __init__(self, object: dict) -> None:
        if "from" in object.keys() and "count" in object.keys():
            self.choose_count = object["count"]
            self.choose_abilities = object["from"]
        else:
            self.choose_count = 1
            self.choose_abilities = object


class Ability:
    strength: int
    dexterity: int
    constitution: int
    intelligence: int
    wisdom: int
    charisma: int
    chooses: list[Choose]

    def __init__(self, object: Optional[dict[str, int | dict | list[dict]]]) -> None:
        self.strength = 0
        self.dexterity = 0
        self.constitution = 0
        self.intelligence = 0
        self.wisdom = 0
        self.charisma = 0
        self.chooses = []

        if object is None:
            return

        self.strength = object["str"] if "str" in object.keys() and object['str'] is int else self.strength
        self.dexterity = object["dex"] if "dex" in object.keys() and object['dex'] is int else self.dexterity
        self.constitution = (
            object["con"] if "con" in object.keys() and object['con'] is int else self.constitution
        )
        self.intelligence = (
            object["int"] if "int" in object.keys() and object['int'] is int else self.intelligence
        )
        self.wisdom = object["wis"] if "wis" in object.keys() and object['wis'] is int else self.wisdom
        self.charisma = object["cha"] if "cha" in object.keys() and object['cha'] is int else self.charisma
        if "choose" in object.keys() and object['choose'] is not int:
            # makes sure it's a list
            chooses: list[dict] = []
            if object['choose'] is dict:
                chooses = [object['choose']]
            elif object['choose'] is list[dict]:
                chooses = object['choose']

            for choose in chooses:
                self.chooses.append(Choose(choose))


class Age:
    mature: Optional[int] = None
    max: Optional[int] = None

    def __init__(self, object: Optional[dict[str, int]]) -> None:
        if object is not None:
            self.mature = object["mature"] if "mature" in object.keys() else self.mature
            self.max = object["max"] if "max" in object.keys() else self.max


class LanguageProficiencies:
    languages: list[str] = []
    any_standard: int = 0
    choose_languages: list[str]
    choose_count: int = 1

    def __init__(
        self, object: Optional[dict[str, bool | int | dict[str, int | list[str]]]]
    ) -> None:
        if object is not None:
            for key in object.keys():
                if type(object[key]) is bool:
                    self.languages.append(key)
                elif type(object[key]) is int:
                    self.any_standard = object[key] # type: ignore
                elif type(object[key]) is dict and type(object[key]['from']) is list and type(object[key]['count']) is int: # type: ignore
                    self.choose_languages = object["choose"]["from"] # type: ignore
                    self.choose_count = object["choose"]["count"] # type: ignore
                else:
                    print("Language proficiency value was not expected")
                    print(f"It was a {type(object[key])} with a value of {object[key]}")


class SkillProficiencies:
    skills: list[str] = []
    choose_skills: list[str] = []
    choose_count: int = 1

    def __init__(
        self, object: Optional[dict[str, bool | dict[str, int | list[str]]]]
    ) -> None:
        if object is None:
            return
            
        for key in object.keys():
            if type(object[key]) is bool:
                self.skills.append(key)
            elif type(object[key]) is dict:
                self.choose_skills = object[key]["from"] # type: ignore
                self.choose_count = (
                    object[key]["count"] # type: ignore
                    if "count" in object[key].keys() # type: ignore
                    else self.choose_count
                )
            else:
                # print(f"Skill proficiency  value was not expected")
                # print(f"It was a {type(object[key])} with a value of {object[key]}")
                pass


class ToolProficiencies:
    tools: list[str] = []
    choose_any_amount: int = 0

    def __init__(self, object: Optional[list[dict[str, int | str]]]) -> None:
        if object is not None:
            for item in object:
                for key in item.keys():
                    if key == "any":
                        self.choose_any_amount = item["any"]
                    else:
                        self.tools.append(key)


class WeaponProficiencies:
    weapons: list[dict[str, str]] = []
    choose_tool_filter: list[str] = []
    choose_amount: int = 0

    def __init__(self, object: Optional[dict[str, bool]]) -> None:
        if object is not None:
            for key in object.keys():
                if key == "choose":
                    self.choose_tools = object[key]["fromFilter"]
                    self.choose_amount = object[key]["count"]
                else:
                    # print(f"key: {key}")
                    if "|" in key:
                        self.weapons.append(
                            {"name": key.split("|")[0], "source": key.split("|")[1]}
                        )
                    else:
                        self.weapons.append({"name": key, "source": "N/A"})

    def has_proficiency(self, name: str) -> bool:
        for item in self.proficiencies:
            if item["name"] == name:
                return True
        return False


class ArmorProficiencies:
    armor: list[str] = []

    def __init__(self, object: Optional[dict[str, bool]]) -> None:
        if object is not None:
            # not checking whether or not it has any because
            # as of Mar 13, the only race that has a armor
            # proficiency just has a defined armor type
            armor = object.keys()


class Source:
    source: str = ""
    page: Optional[int] = -1

    def __init__(
        self,
        object: Optional[dict[str, str | int]] = None,
        source: str = None,
        page: int = None,
    ) -> None:
        if source is not None and page is not None:
            self.source = source
            self.page = page
        elif object is not None:
            self.source = object["source"]
            self.page = object["page"] if "page" in object.keys() else self.page


class HeightAndWeight:
    base_height: int = None
    height_mod: str = None
    base_weight: int = None
    weight_mod: str = None

    def __init__(self, object: Optional[dict[str, int | str]]) -> None:
        if object is not None:
            self.base_height = object["baseHeight"]
            self.height_mod = (
                object["heightMod"] if "heightMod" in object.keys() else self.height_mod
            )
            self.base_weight = object["baseWeight"]
            self.weight_mod = (
                object["weightMod"] if "weightMod" in object.keys() else self.weight_mod
            )


class Resist:
    resists: list[str] = []
    choose_from: list[str] = []

    def __init__(self, object: Optional[list[str | dict[str, list]]]) -> None:
        if object is not None:
            for item in object:
                if type(item) is list:
                    self.resists.append(item)
                elif type(item) is dict[str, list]:
                    self.choose_from = item["from"]


class Reset(StrEnum):
    NEVER = "never"
    REST = "rest"
    DAILY = "daily"


class AdditionalSpells:
    spells: list[dict[str, any]] = []
    choose: list[dict[str, str]] = []

    def __init__(self, object: Optional[list[dict[str, dict[str, any]]]]) -> None:
        if object is None:
            return

        for spell_object in object:
            if "innate" in spell_object.keys():
                for level in spell_object["innate"].keys():
                    if type(spell_object["innate"][level]) is list:
                        for spell in spell_object["innate"][level]:
                            self.spells.append(
                                {
                                    "name": spell,
                                    "ability": spell_object["ability"],
                                    "reset_when": Reset.NEVER,
                                    "aquired_at": int(level),
                                }
                            )
                    elif type(spell_object["innate"][level]) is dict:
                        if "daily" in spell_object["innate"][level].keys():
                            for spell in spell_object["innate"][level]["daily"]["1"]:
                                self.spells.append(
                                    {
                                        "name": spell,
                                        "ability": spell_object["ability"],
                                        "reset_when": Reset.DAILY,
                                        "aquired_at": int(level),
                                    }
                                )
                        else:
                            print(
                                "spell_object['innate'][level] was a dict with an unknown key"
                            )
            if "known" in spell_object.keys():
                for level in spell_object["known"].keys():
                    # for now, the only thing that has this is koblod from MPMM so its hard coded
                    if level == "_":
                        # still including the for loop here just incase
                        for spell in spell_object["known"]["_"]:
                            self.choose.append(
                                {
                                    "spell_list": spell["choose"][
                                        spell["choose"]
                                        .find("level=") : spell["choose"]
                                        .find("|")
                                    ],
                                    "count": spell["choose"][
                                        spell["choose"].find("class=") :
                                    ],
                                    "ability": spell_object["ability"],
                                    "aquired_at": 0,
                                }
                            )
                    else:
                        if type(spell_object["known"][level]) is list:
                            for spell in spell_object["known"][level]:
                                self.spells.append(
                                    {
                                        "name": spell,
                                        "ability": (
                                            spell_object["ability"]
                                            if "ability" in spell_object.keys()
                                            else "base"
                                        ),
                                        "reset_when": Reset.NEVER,
                                        "aquired_at": int(level),
                                    }
                                )
                        elif type(spell_object["known"][level]) is dict:
                            if "rest" in spell_object["known"][level].keys():
                                for spell in spell_object["known"][level]["rest"]:
                                    self.spells.append(
                                        {
                                            "name": spell,
                                            "ability": (
                                                spell_object["ability"]
                                                if "ability" in spell_object.keys()
                                                else "base"
                                            ),
                                            "reset_when": Reset.REST,
                                            "aquired_at": int(level),
                                        }
                                    )
                            else:
                                print(
                                    "spell_object['known'] was a dict with an unknown key"
                                )

    def new_spells_at(self, level: int) -> list[str]:
        new_spells: list[str] = []
        for spell in self.spells:
            if spell["aquired_at"] == level:
                new_spells.append(spell["name"])
        return new_spells


class Time:
    quantity: int
    unit: str

    def __init__(self, object: dict) -> None:
        self.quantity = object["number"]
        self.unit = object["unit"]


class Range:
    type: str
    form: str
    amount: int

    def __init__(self, object: dict) -> None:
        self.type = object["type"]
        if self.type == "special":
            self.form = ""
            self.amount = -1
            return
        self.form = object["distance"]["type"]
        if self.form not in ["self", "touch", "sight", "unlimited"]:
            self.amount = object["distance"]["amount"]
            return
        self.amount = -1


class Components:
    v: bool
    s: bool
    m: str
    r: str

    def __init__(self, object: dict) -> None:
        # impl 1
        # v = "v" in object.keys()
        # s = "s" in object.keys()
        # m = object["m"] if "m" in object.keys() else ""

        # impl 2
        self.v = False
        self.s = False
        self.m = ""
        self.r = ""

        for key in object.keys():
            if key == "v":
                self.v = True
            elif key == "s":
                self.s = True
            elif key == "m":
                self.m = object["m"]
            elif key == "r":
                self.r = object["r"]
            else:
                print(key)


class Duration:
    type: str
    unit: str
    amount: int
    concentration: bool
    ends: list[str]

    def __init__(self, object: dict) -> None:
        self.type = object["type"]
        self.ends = []
        self.concentration = (
            object["concentration"] if "concentration" in object.keys() else False
        )
        if "duration" in object.keys():
            self.unit = object["duration"]["type"]
            self.amount = object["duration"]["amount"]
        else:
            self.amount = 0
            self.unit = ""
        self.ends = object["ends"] if "ends" in object.keys() else []


class ScalingLevelDice:
    type: str
    scaling: dict[int, str]

    def __init__(self, object: dict) -> None:
        self.type = object["label"][: object["label"].find(" ")]
        self.scaling = object["scaling"]


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
    """Currently Not Used"""

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
    BULLET_SLING = "bulletSling"
    NEITHER = "item"

    def __init__(self, name) -> None:
        super().__init__()


class ContainerSlot:
    weight_limit: int
    valid_items: dict[str, int]

    def __init__(self, weight: int, item_info: dict[str, int]) -> None:
        self.weight_limit = weight
        self.valid_items = item_info


class ContainerCapacity:
    slots: list[ContainerSlot]
    weightless: bool

    def __init__(self, object: Optional[dict]) -> None:
        self.slots = []

        if object is None:
            self.weightless = False
            return

        weight_length = len(object["weight"]) if "weight" in object.keys() else 0
        item_length = len(object["item"]) if "item" in object.keys() else 0
        if weight_length == 0:
            object["weight"] = [-1] * item_length
        elif item_length == 0:
            object["item"] = [{}] * weight_length
        else:
            if weight_length > item_length:
                object["item"].extend([{}] * (weight_length - item_length))
            elif item_length > weight_length:
                object["weight"].extend([-1] * (item_length - weight_length))

        for weight, item in zip(object["weight"], object["item"]):
            self.slots.append(ContainerSlot(weight, item))

        self.weightless = True if "weightless" in object.keys() else False

        self.__dict__["slots"] = [slot.__dict__ for slot in self.slots]
