import json
import datetime
from typing import Any
from enum import StrEnum
from src.DAO.MiscClasses import PackItem, Source, Ability, ContainerCapacity


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
    firearm: bool
    weapon: bool
    ammo_type: str
    srd: bool
    basic_rules: bool
    value: int
    arrow: bool
    pack_contents: list[PackItem]
    dmg2: str
    axe: bool
    entries: list[dict[str, list[str] | str]] | str
    ac: int
    armor: bool
    strength: int
    stealth: bool
    club: bool
    bolt: bool
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
    bullet_sling: bool

    def __init__(self, object: dict) -> None:
        # print(object)
        self.name = object["name"]
        self.source = object["source"]
        self.page = object["page"] if "page" in object.keys() else -1
        self.type = object["type"] if "type" in object.keys() else ""
        self.rarity = object["rarity"] if "rarity" in object.keys() else "none"
        self.weight = object["weight"] if "weight" in object.keys() else 0
        self.weapon_category = object["weaponCategory"] if "weaponCategory" in object.keys() else ""
        self.age = object["age"] if "age" in object.keys() else ""
        self.property = object["property"] if "property" in object.keys() else []
        self.range = object["range"] if "range" in object.keys() else ""
        self.reload = object["reload"] if "reload" in object.keys() else 0
        self.dmg1 = object["dmg1"] if "dmg1" in object.keys() else ""
        self.dmg_type = object["dmgType"] if "dmgType" in object.keys() else ""
        self.weapon = object["weapon"] if "weapon" in object.keys() else False
        self.ammo_type = object["ammoType"] if "ammoType" in object.keys() else ""
        self.srd = object["srd"] if "srd" in object.keys() else False
        self.basic_rules = object["basicRules"] if "basicRules" in object.keys() else False
        self.value = object["value"] if "value" in object.keys() else -1
        self.pack_contents = (
            [PackItem(item) for item in object["packContents"]] if "packContents" in object.keys() else []
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
        self.bullet_sling = object["bulletSling"] if "bulletSling" in object.keys() else False
        # self.item_type = self.get_item_type(object)

    def as_dict(self) -> dict[str, Any]:
        """This is effectivaly read only as it creates a copy

        Returns:
            dict[str, Any]: the class as a dict
        """

        copy = self.__dict__.copy()
        copy["pack_contents"] = [pack_content.__dict__ for pack_content in copy["pack_contents"]]
        return copy

    # def get_item_type(self, object: dict) -> ItemType:
    #     types = [
    #         "firearm"
    #         "arrow"
    #         "axe"
    #         "armor"
    #         "club"
    #         "bolt"
    #         "dagger"
    #         "sword"
    #         "polearm"
    #         "crossbow"
    #         "spear"
    #         "hammer"
    #         "bow"
    #         "mace"
    #         "net"
    #         "staff"
    #     ]

    #     valid_types = []

    #     # impl 1
    #     # for type in types:
    #     #     if type in object.keys():
    #     #         valid_types.append(type)
    #     # impl 2
    #     for key in object.keys():
    #         for _ in range(0, len(types)):
    #             if types[0] == key:
    #                 valid_types.append(key)
    #             types.pop(0)
    #     if len(valid_types) > 1:
    #         print(f"there are more than one valid_types: {valid_types}")
    #     elif len(valid_types) == 0:
    #         return ItemType("item")
    #     return ItemType(valid_types[0])


class MagicItem:
    name: str
    source: str
    page: int
    rarity: str
    req_attune: str
    req_attune_tags: list[str]
    wondrous: bool
    bonus_spell_attack: str
    bonus_spell_save_DC: str
    focus: list[str]
    entries: list[dict[str, list[str] | str]] | str
    weight: int
    has_fluff_Images: bool
    base_item: str
    type: str
    weapon_category: str
    property: list[str]
    dmg1: str
    dmg_type: str
    bonus_weapon: str
    tier: str
    loot_tables: list[str]
    srd: bool | str
    # copy: Copy
    bonus_AC: str
    bonus_saving_throw: str
    optionalfeatures: list[str]
    resist: list[str]
    ac: int
    basic_rules: bool
    value: int
    recharge: str
    recharge_amount: str
    charges: int
    misc_tags: list[str]
    detail1: str
    tattoo: bool
    has_refs: bool
    attached_spells: list[str]
    crew: int
    veh_AC: int
    veh_HP: int
    veh_speed: int
    cap_passenger: int
    cap_cargo: int
    condition_immune: list[str]
    dmg2: str
    grants_proficiency: bool
    additional_sources: list[Source]
    additional_entries: list[dict[str, list[str] | str]] | str
    # modify_speed: ModifySpeed # i don't really need this for this application
    scf_type: str
    curse: bool
    ability: Ability
    see_also_vehicle: list[str]
    range: str
    strength: str
    stealth: bool
    immune: list[str]
    vulnerable: list[str]
    poison: bool
    poison_types: list[str]
    sentient: bool
    container_capacity: ContainerCapacity
    pack_contents: str  # take from other DAO
    atomic_pack_contents: bool
    bonus_weapon_attack: str
    other_sources: list[Source]
    grants_language: bool
    staff: bool
    age: str
    veh_dmg_thresh: int
    bonus_weapon_damage: str
    crit_threshold: int
    carrying_capacity: str
    speed: int
    ammo_type: str  # item|source
    alias: list[str]
    see_also_deck: list[str]
    reprinted_as: list[str]
    has_fluff: bool
    req_attune_alt: str
    reach: int
    bonus_proficiency_bonus: str
    firearm: bool
    bonus_saving_throw_concentration: str
    type_alt: str
    # dexterity_max: str # this is only used by Serpent Scale Armor|CM and it's null
    crew_min: int
    crew_max: int
    travel_cost: int
    shipping_cost: int
    spell_scroll_level: int
    bonus_ability_check: str
    weight_note: str

    def __init__(self, object: dict) -> None:
        self.name = object["name"] if "name" in object.keys() else ""
        self.source = object["source"] if "source" in object.keys() else ""
        self.page = object["page"] if "page" in object.keys() else -1
        self.rarity = object["rarity"] if "rarity" in object.keys() else ""
        self.req_attune = object["reqAttune"] if "reqAttune" in object.keys() else ""
        self.req_attune_tags = object["reqAttuneTags"] if "reqAttuneTags" in object.keys() else []
        self.wondrous = object["wondrous"] if "wondrous" in object.keys() else False
        self.bonus_spell_attack = object["bonusSpellAttack"] if "bonusSpellAttack" in object.keys() else ""
        self.bonus_spell_save_DC = object["bonusSpellSaveDc"] if "bonusSpellSaveDc" in object.keys() else ""
        self.focus = object["focus"] if "focus" in object.keys() else []
        self.entries = object["entries"] if "entries" in object.keys() else ""
        self.weight = object["weight"] if "weight" in object.keys() else -1
        self.has_fluff_Images = object["hasFluffImages"] if "hasFluffImages" in object.keys() else False
        self.base_item = object["baseItem"] if "baseItem" in object.keys() else ""
        self.type = object["type"] if "type" in object.keys() else ""
        self.weapon_category = object["weaponCategory"] if "weaponCategory" in object.keys() else ""
        self.property = object["property"] if "property" in object.keys() else []
        self.dmg1 = object["dmg1"] if "dmg1" in object.keys() else ""
        self.dmg_type = object["dmgType"] if "dmgType" in object.keys() else ""
        self.bonus_weapon = object["bonusWeapon"] if "bonusWeapon" in object.keys() else ""
        self.tier = object["tier"] if "tier" in object.keys() else ""
        self.loot_tables = object["lootTables"] if "lootTables" in object.keys() else []
        self.srd = object["srd"] if "srd" in object.keys() else ""
        self.copy = object["_copy"] if "_copy" in object.keys() else ""
        self.bonus_AC = object["bonusAc"] if "bonusAc" in object.keys() else ""
        self.bonus_saving_throw = object["bonusSavingThrow"] if "bonusSavingThrow" in object.keys() else ""
        self.optionalfeatures = object["optionalfeatures"] if "optionalfeatures" in object.keys() else []
        self.resist = object["resist"] if "resist" in object.keys() else []
        self.ac = object["ac"] if "ac" in object.keys() else -1
        self.basic_rules = object["basicRules"] if "basicRules" in object.keys() else False
        self.value = object["value"] if "value" in object.keys() else -1
        self.recharge = object["recharge"] if "recharge" in object.keys() else ""
        self.recharge_amount = object["rechargeAmount"] if "rechargeAmount" in object.keys() else ""
        self.charges = object["charges"] if "charges" in object.keys() else -1
        self.misc_tags = object["miscTags"] if "miscTags" in object.keys() else []
        self.detail1 = object["detail1"] if "detail1" in object.keys() else ""
        self.tattoo = object["tattoo"] if "tattoo" in object.keys() else False
        self.has_refs = object["hasRefs"] if "hasRefs" in object.keys() else False
        self.attached_spells = object["attachedSpells"] if "attachedSpells" in object.keys() else []
        self.crew = object["crew"] if "crew" in object.keys() else -1
        self.veh_AC = object["vehAc"] if "vehAc" in object.keys() else -1
        self.veh_HP = object["vehHp"] if "vehHp" in object.keys() else -1
        self.veh_speed = object["vehSpeed"] if "vehSpeed" in object.keys() else -1
        self.cap_passenger = object["capPassenger"] if "capPassenger" in object.keys() else -1
        self.cap_cargo = object["capCargo"] if "capCargo" in object.keys() else -1
        self.condition_immune = object["conditionImmune"] if "conditionImmune" in object.keys() else []
        self.dmg2 = object["dmg2"] if "dmg2" in object.keys() else ""
        self.grants_proficiency = object["grantsProficiency"] if "grantsProficiency" in object.keys() else False
        self.additional_sources = (
            [Source(source) for source in object["additionalSources"]] if "additionalSources" in object.keys() else []
        )
        self.additional_entries = object["additionalEntries"] if "additionalEntries" in object.keys() else ""
        self.modify_speed = object["modifySpeed"] if "modifySpeed" in object.keys() else ""
        self.scf_type = object["scfType"] if "scfType" in object.keys() else ""
        self.curse = object["curse"] if "curse" in object.keys() else False
        self.ability = Ability(object["ability"]) if "ability" in object.keys() else Ability(None)
        self.see_also_vehicle = object["seeAlsoVehicle"] if "seeAlsoVehicle" in object.keys() else []
        self.range = object["range"] if "range" in object.keys() else ""
        self.strength = object["strength"] if "strength" in object.keys() else ""
        self.stealth = object["stealth"] if "stealth" in object.keys() else False
        self.immune = object["immune"] if "immune" in object.keys() else []
        self.vulnerable = object["vulnerable"] if "vulnerable" in object.keys() else []
        self.poison = object["poison"] if "poison" in object.keys() else False
        self.poison_types = object["poisonTypes"] if "poisonTypes" in object.keys() else []
        self.sentient = object["sentient"] if "sentient" in object.keys() else False
        self.container_capacity = (
            ContainerCapacity(object["containerCapacity"])
            if "containerCapacity" in object.keys()
            else ContainerCapacity(None)
        )
        self.pack_contents = object["packContents"] if "packContents" in object.keys() else ""
        self.atomic_pack_contents = object["atomicPackContents"] if "atomicPackContents" in object.keys() else False
        self.bonus_weapon_attack = object["bonusWeaponAttack"] if "bonusWeaponAttack" in object.keys() else ""
        self.other_sources = (
            [Source(source) for source in object["otherSources"]] if "otherSources" in object.keys() else []
        )
        self.grants_language = object["grantsLanguage"] if "grantsLanguage" in object.keys() else False
        self.staff = object["staff"] if "staff" in object.keys() else False
        self.age = object["age"] if "age" in object.keys() else ""
        self.veh_dmg_thresh = object["vehDmgThresh"] if "vehDmgThresh" in object.keys() else -1
        self.bonus_weapon_damage = object["bonusWeaponDamage"] if "bonusWeaponDamage" in object.keys() else ""
        self.crit_threshold = object["critThreshold"] if "critThreshold" in object.keys() else -1
        self.carrying_capacity = object["carryingCapacity"] if "carryingCapacity" in object.keys() else ""
        self.speed = object["speed"] if "speed" in object.keys() else -1
        self.ammo_type = object["ammoType"] if "ammoType" in object.keys() else ""
        self.alias = object["alias"] if "alias" in object.keys() else []
        self.see_also_deck = object["seeAlsoDeck"] if "seeAlsoDeck" in object.keys() else []
        self.reprinted_as = object["reprintedAs"] if "reprintedAs" in object.keys() else []
        self.has_fluff = object["hasFluff"] if "hasFluff" in object.keys() else False
        self.req_attune_alt = object["reqAttuneAlt"] if "reqAttuneAlt" in object.keys() else ""
        self.reach = object["reach"] if "reach" in object.keys() else -1
        self.bonus_proficiency_bonus = (
            object["bonusProficiencyBonus"] if "bonusProficiencyBonus" in object.keys() else ""
        )
        self.firearm = object["firearm"] if "firearm" in object.keys() else False
        self.bonus_saving_throw_concentration = (
            object["bonusSavingThrowConcentration"] if "bonusSavingThrowConcentration" in object.keys() else ""
        )
        self.type_alt = object["typeAlt"] if "typeAlt" in object.keys() else ""
        self.crew_min = object["crewMin"] if "crewMin" in object.keys() else -1
        self.crew_max = object["crewMax"] if "crewMax" in object.keys() else -1
        self.travel_cost = object["travelCost"] if "travelCost" in object.keys() else -1
        self.shipping_cost = object["shippingCost"] if "shippingCost" in object.keys() else -1
        self.spell_scroll_level = object["spellScrollLevel"] if "spellScrollLevel" in object.keys() else -1
        self.bonus_ability_check = object["bonusAbilityCheck"] if "bonusAbilityCheck" in object.keys() else ""
        self.weight_note = object["weightNote"] if "weightNote" in object.keys() else ""

    def as_dict(self) -> dict[str, Any]:
        """This is effectivaly read only as it creates a copy

        Returns:
            dict[str, Any]: the class as a dict
        """

        copy = self.__dict__.copy()
        copy["additional_sources"] = [source.__dict__ for source in copy["additional_sources"]]
        copy["ability"] = copy["ability"].__dict__
        copy["container_capacity"] = copy["container_capacity"].as_dict()
        copy["other_sources"] = [source.__dict__ for source in copy["other_sources"]]
        return copy


class Items:
    mundane_items: dict[str, MundaneItem]
    magic_items: dict[str, MagicItem]

    def __init__(self) -> None:
        mundane_f = open("data/raw/items-base.json")
        magic_f = open("data/raw/items.json")

        mundane_object = json.load(mundane_f)
        magic_object = json.load(magic_f)

        self.mundane_items = {}
        self.magic_items = {}

        for mundane_item in mundane_object["baseitem"]:
            self.mundane_items[mundane_item["name"]] = MundaneItem(mundane_item)

        for magic_item in magic_object["item"]:
            self.magic_items[magic_item["name"]] = MagicItem(magic_item)

        mundane_f.close()
        magic_f.close()

    def as_dict(self) -> dict[str, Any]:
        """This is effectivaly read only as it creates a copy

        Returns:
            dict[str, Any]: the class as a dict
        """

        copy = self.__dict__.copy()
        for item_name in copy['mundane_items'].keys():
            copy['mundane_items'][item_name] = copy["mundane_items"][item_name].as_dict()
        for item_name in copy['magic_items'].keys():
            copy['magic_items'][item_name] = copy["magic_items"][item_name].as_dict()
        return copy


if __name__ == "__main__":
    times = []
    count = 1000
    for _ in range(0, count):
        start = datetime.datetime.now()
        items = Items()
        end = datetime.datetime.now()
        times.append((end - start).microseconds)
        del items

    average_time = sum(times) / count

    print(f"average time was {average_time} microseconds")
