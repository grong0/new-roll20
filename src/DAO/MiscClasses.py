from enum import StrEnum
from typing import Any, Optional


class Speed:
    walk: int
    fly: Optional[int] = None
    swim: Optional[int] = None
    climb: Optional[int] = None

    def __init__(self, object: int | dict[str, int]) -> None:
        if type(object) is int:
            self.walk = object
        elif type(object) is dict:
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

        self.strength = object["str"] if "str" in object.keys() and object["str"] is int else self.strength
        self.dexterity = object["dex"] if "dex" in object.keys() and object["dex"] is int else self.dexterity
        self.constitution = object["con"] if "con" in object.keys() and object["con"] is int else self.constitution
        self.intelligence = object["int"] if "int" in object.keys() and object["int"] is int else self.intelligence
        self.wisdom = object["wis"] if "wis" in object.keys() and object["wis"] is int else self.wisdom
        self.charisma = object["cha"] if "cha" in object.keys() and object["cha"] is int else self.charisma
        if "choose" in object.keys() and object["choose"] is not int:
            # makes sure it's a list
            chooses: list[dict] = []
            if object["choose"] is dict:
                chooses = [object["choose"]]
            elif object["choose"] is list[dict]:
                chooses = object["choose"]

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
    languages: list[str]
    any_standard: int
    choose_languages: list[str]
    choose_count: int

    def __init__(self, object: Optional[dict[str, bool | int | dict[str, int | list[str]]]]) -> None:
        self.languages = []
        self.any_standard = 0
        self.choose_languages = []
        self.choose_count = 1

        if object is None:
            return

        for key in object.keys():
            if type(object[key]) is bool:
                self.languages.append(key)
            elif type(object[key]) is int:
                self.any_standard = object[key]  # type: ignore
            elif type(object[key]) is dict and type(object[key]["from"]) is list:  # type: ignore
                if "count" in object[key].keys():  # type: ignore
                    self.choose_count = object[key]["count"]  # type: ignore
                self.choose_languages = object["choose"]["from"]  # type: ignore
            else:
                print("Language proficiency value was not expected")
                print(f"It was a {type(object[key])} with a value of {object[key]}")


class SkillProficiencies:
    skills: list[str] = []
    choose_skills: list[str] = []
    choose_count: int = 1

    def __init__(self, object: Optional[dict[str, bool | dict[str, int | list[str]]]]) -> None:
        if object is None:
            return

        for key in object.keys():
            if type(object[key]) is bool:
                self.skills.append(key)
            elif type(object[key]) is dict:
                self.choose_skills = object[key]["from"]  # type: ignore
                self.choose_count = (  # type: ignore
                    object[key]["count"]  # type: ignore
                    if "count" in object[key].keys()  # type: ignore
                    else self.choose_count
                )
            else:
                # print(f"Skill proficiency  value was not expected")
                # print(f"It was a {type(object[key])} with a value of {object[key]}")
                pass


class ToolProficiencies:
    tools: list[str]
    choose_any_amount: int

    def __init__(self, object: Optional[list[dict[str, int | str]]]) -> None:
        self.tools = []
        self.choose_any_amount = 0

        if object is None:
            return

        for item in object:
            for key in item.keys():
                if key == "any" and type(item["any"]) is str:
                    self.choose_any_amount = item["any"]  # type: ignore
                else:
                    self.tools.append(key)


class WeaponProficiencies:
    weapons: list[dict[str, str]]
    choose_tool_filter: list[str]
    choose_amount: int

    def __init__(self, object: Optional[dict[str, bool | dict]]) -> None:
        self.weapons = []
        self.choose_tool_filter = []

        if object is None:
            self.choose_amount = 0
            return

        for key in object.keys():
            if key == "choose":
                self.choose_tool_filter = object[key]["fromFilter"]  # type: ignore
                self.choose_amount = object[key]["count"]  # type: ignore
            else:
                # print(f"key: {key}")
                if "|" in key:
                    self.weapons.append({"name": key.split("|")[0], "source": key.split("|")[1]})
                else:
                    self.weapons.append({"name": key, "source": "N/A"})

    def has_proficiency(self, name: str) -> bool:
        for item in self.weapons:
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
            self.armor = list(object.keys())


class Source:
    source: str
    page: Optional[int]

    def __init__(
        self,
        object: Optional[dict[str, str | int]] = None,
        source: Optional[str] = None,
        page: Optional[int] = None,
    ) -> None:
        self.source = ""
        self.page = -1

        if source is not None and page is not None:
            self.source = source
            self.page = page
        elif object is not None:
            self.source = object["source"]  # type: ignore
            self.page = object["page"] if "page" in object.keys() else self.page  # type: ignore


class HeightAndWeight:
    base_height: int
    height_mod: str
    base_weight: int
    weight_mod: str

    def __init__(self, object: Optional[dict[str, int | str]]) -> None:
        if object is None:
            self.base_height = -1
            self.height_mod = ""
            self.base_weight = -1
            self.weight_mod = ""
            return

        self.base_height = object["baseHeight"]  # type: ignore
        self.height_mod = object["heightMod"] if "heightMod" in object.keys() else ""  # type: ignore
        self.base_weight = object["baseWeight"]  # type: ignore
        self.weight_mod = object["weightMod"] if "weightMod" in object.keys() else ""  # type: ignore


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
    spells: list[dict[str, Any]] = []
    choose: list[dict[str, Any]] = []

    def __init__(self, object: Optional[list[dict[str, dict[str, Any]]]]) -> None:
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
                            print("spell_object['innate'][level] was a dict with an unknown key")
            if "known" in spell_object.keys():
                for level in spell_object["known"].keys():
                    # for now, the only thing that has this is koblod from MPMM so its hard coded
                    if level == "_":
                        # still including the for loop here just incase
                        for spell in spell_object["known"]["_"]:
                            self.choose.append(
                                {
                                    "spell_list": spell["choose"][
                                        spell["choose"].find("level=") : spell["choose"].find("|")
                                    ],
                                    "count": spell["choose"][spell["choose"].find("class=") :],
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
                                            spell_object["ability"] if "ability" in spell_object.keys() else "base"
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
                                                spell_object["ability"] if "ability" in spell_object.keys() else "base"
                                            ),
                                            "reset_when": Reset.REST,
                                            "aquired_at": int(level),
                                        }
                                    )
                            else:
                                print("spell_object['known'] was a dict with an unknown key")

    def new_spells_at(self, level: int) -> list[str]:
        new_spells: list[str] = []
        for spell in self.spells:
            if spell["aquired_at"] == level:
                new_spells.append(spell["name"])
        return new_spells


class Time:
    quantity: int
    unit: str

    def __init__(self, object: Optional[dict]) -> None:
        if object is None:
            self.quantity = 0
            self.unit = ""
            return
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
        self.concentration = object["concentration"] if "concentration" in object.keys() else False
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
        self.is_special = False
        if type(object) == dict:
            if "special" in object.keys():
                self.name = object["special"]
                self.is_special = True
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

    def as_dict(self) -> dict[str, Any]:
        """This is effectivaly read only as it creates a copy

        Returns:
            dict[str, Any]: the class as a dict
        """

        copy = self.__dict__.copy()
        copy["slots"] = [slot.__dict__ for slot in copy["slots"]]
        return copy


class StartingItem:
    name: str
    quantity: int
    value: int
    contains_value: int
    display_name: str
    is_pouch: bool
    is_special: bool
    is_equipment_type: bool
    is_money: bool

    def __init__(self, item: str | dict) -> None:
        self.quantity = 1
        self.value = -1
        self.contains_value = 0
        self.display_name = ""
        self.is_pouch = False
        self.is_special = False
        self.is_equipment_type = False
        self.is_money = False

        if type(item) is str:
            self.name = item
            return

        if type(item) is not dict:
            print(f"found type other than str and dict: {type(item)}")
            print(item, "\n")
            self.name = ""
            return

        for key in item.keys():
            match key:
                case "item":
                    self.name = item[key]
                case "special":
                    self.name = item[key]
                    self.is_special = True
                case "worthValue":
                    self.value = item[key]
                case "containsValue":
                    self.is_pouch = True
                    self.contains_value = item[key]
                case "quantity":
                    self.quantity = item[key]
                case "equipmentType":
                    self.name = item[key]
                    self.is_equipment_type = True
                case "displayName":
                    self.display_name = item[key]
                case "value":
                    self.value = item[key]
                    self.is_money = True
                case _:
                    print(f"found key not accounted for: {key} = {item[key]}")

        # self.name = item['item']
        # if 'quantity' in item.keys(): self.quantity = item['quantity']
        # if 'containsValue' in item.keys():
        #     self.pouch = True
        #     self.contains_value = item['containsValue']


class StartingEquipment:
    """
    list of dict with keys of 'a', 'b', and '_'
    """

    items: list[StartingItem]
    choose_between: list[list[StartingItem]]

    def __init__(self, object: list[dict[str, Any]]) -> None:
        self.items = []
        self.choose_between = []

        for category in object:
            if "_" in category.keys():
                for item in category["_"]:
                    # if type(item) is str:
                    #     self.items.append(item)
                    # elif type(item) is dict:
                    #     if "special" in item.keys():
                    #         self.is_special_items = item['special']
                    #     elif 'item' in item.keys() and item['item'] == "pouch|phb":
                    #         self.pouch = True
                    #         self.money = item['containsValue']
                    # else:
                    #     print(f"type not accounting for: {type(item)}")
                    #     print(item)

                    if type(item) is list:
                        self.items.extend([StartingItem(item) for item in item])
                        continue

                    self.items.append(StartingItem(item))
                continue

            if "c" in category.keys():
                items = zip(category["a"], category["b"], category["c"])
            else:
                items = zip(category["a"], category["b"])

            for items_zipped in items:
                classed_items = [StartingItem(item) for item in items_zipped]
                self.choose_between.append(classed_items)

    def as_dict(self) -> dict[str, Any]:
        """This is effectivaly read only as it creates a copy

        Returns:
            dict[str, Any]: the class as a dict
        """

        copy = self.__dict__.copy()
        copy["items"] = [item.__dict__ for item in self.items]
        copy["choose_between"] = [[item.__dict__ for item in choose] for choose in self.choose_between]
        return copy


class ClassPrerequisite:
    name: str
    level: int
    visible: bool

    def __init__(self, object: dict[str, Any]) -> None:
        self.name = object["class"]["name"]
        self.level = object["level"]
        self.visible = object["class"]["visible"]


class Prerequisite:  # TODO: add support for choice from, just level, ability, summaries?, feats, other,
    campaign_requirement: list[str]
    requires_campaign: bool
    class_requirement: list[ClassPrerequisite]
    requires_class: bool

    def __init__(self, list: Optional[list[dict[str, Any]]]) -> None:
        self.campaign_requirement = []
        self.class_requirement = []

        if list is None:
            self.requires_campaign = False
            self.requires_class = False
            return

        for object in list:
            if "level" in object.keys():
                self.class_requirement.append(ClassPrerequisite(object["level"]))
                self.requires_class = True
            if "campaign" in object.keys():
                self.campaign_requirement.append(object["campaign"])
                self.requires_campaign = True

    def as_dict(self) -> dict[str, Any]:
        """This is effectivaly read only as it creates a copy

        Returns:
            dict[str, Any]: the class as a dict
        """

        copy = self.__dict__.copy()
        copy["class_requirement"] = [class_level.__dict__ for class_level in self.class_requirement]
        return copy


class SkillToolLanguageChoice:
    language_amount: int
    tool_amount: int
    global_count: int

    def __init__(self, object: dict[str, Any]) -> None:
        self.language_amount = object["anyLanguage"] if "anyLanguage" in object.keys() else 0
        self.tool_amount = object["anyTool"] if "anyTool" in object.keys() else 0


class SkillToolLanguageProficiencies:
    choices: list[SkillToolLanguageChoice]

    def __init__(self, object: Optional[list[dict[str, Any]]]) -> None:
        self.choices = []

        if object is None:
            return

        for choice in object:
            self.choices.append(SkillToolLanguageChoice(choice))

    def as_dict(self) -> dict[str, Any]:
        """This is effectivaly read only as it creates a copy

        Returns:
            dict[str, Any]: the class as a dict
        """

        copy = self.__dict__.copy()
        copy["choices"] = [choice.__dict__ for choice in self.choices]
        return copy


class OptionalFeatureProgression:
    """_summary_
    Only found in four features
    (Eldritch Adept, Fighting Style,
    Martial Adept, Metamagic Adept)
    """

    name: str
    feature_type: list[str]
    progression_type: str
    progression_amount: int

    def __init__(self, object: Optional[dict]) -> None:
        if object is None:
            self.name = ""
            self.feature_type = []
            self.progression_type = ""
            self.progression_amount = 0
            return

        self.name = object["name"]
        self.feature_type = object["featureType"]
        self.progression_type = list(object["progression"].keys())[0]
        self.progression_amount = object["progression"][self.progression_type]


class SavingThrowProficiencies:
    """
    Only found in one feat (Resilient)
    so not a lot to go off of
    """

    options: list[str]
    amount: int

    def __init__(self, object: Optional[dict]) -> None:
        self.options = []

        if object is None:
            self.amount = 0
            return

        self.options = object["from"]
        self.amount = object["count"] if "count" in object.keys() else 1


class Expertise:
    """
    Only found in one feat (Skill Expert)
    so not a lot to go off of
    """

    skill: str
    can_choose: bool
    amount: int

    def __init__(self, object: Optional[dict]) -> None:
        if object is None or object[0] != "anyProficientSkill":
            self.skill = ""
            self.can_choose = False
            self.amount = 0
            return

        self.skill = object[0]
        self.can_choose = True
        self.amount = object[self.skill]
