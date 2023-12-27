import asyncio
from prisma import Prisma
import requests
from prettyprinter import pprint


class Data:
    core_url: str
    damage_types: list[dict[str, str | list[str]]]

    def __init__(self) -> None:
        self.damage_types = []
        self.core_url = "https://www.dnd5eapi.co"
        self.response = requests.get(self.core_url + "/api/").json()

        self.ability_scores = self.parse_request("ability-scores")
        self.alignments = self.parse_request("alignments")
        # self.backgrounds = self.parse_request("backgrounds")
        # self.classes = self.parse_request("classes")
        self.conditions = self.parse_request("conditions")
        self.damage_types = self.parse_request("damage-types")
        # self.equipment = self.parse_request("equipment")
        self.equipment_categories = self.parse_request("equipment-categories")
        # self.feats = self.parse_request("feats")
        # self.languages = self.parse_request("languages")
        # self.magic_items = self.parse_request("languages")
        self.magic_schools = self.parse_request("magic-schools")
        # self.monsters = self.parse_request("monsters")
        # self.proficiencies = self.parse_request("proficiencies")
        # self.races = self.parse_request("races")
        self.rule_sections = self.parse_request("rule-sections")
        self.rules = self.parse_request("rules")
        self.skills = self.parse_request("skills")
        # self.spells = self.parse_request("spells")
        # self.subclasses = self.parse_request("subclasses")
        # self.subraces = self.parse_request("subraces")
        # self.traits = self.parse_request("traits")
        self.weapon_properties = self.parse_request("weapon-properties")

    def parse_request(self, key: str):
        link = self.response[key]
        references = requests.get(self.core_url + link).json()
        results = []
        for reference in references["results"]:
            results.append(requests.get(self.core_url + reference["url"]).json())
        print(f"finished requesting {key}")
        return results

    def get_ability_scores(self):
        return self.ability_scores

    def get_alignments(self):
        return self.alignments

    def get_conditions(self):
        return self.conditions

    def get_damage_types(self) -> list[dict[str, str | list[str]]]:
        return self.damage_types

    def get_equipment_categories(self):
        return self.equipment_categories

    def get_magic_schools(self):
        return self.magic_schools

    def get_rule_sections(self):
        return self.rule_sections

    def get_rules(self):
        return self.rules

    def get_skills(self):
        return self.skills

    def get_weapon_properties(self):
        return self.weapon_properties


async def upsert(model, data):
    current_object = await model.find_first(where={"name": {"equals": data["name"]}})
    if current_object is None:
        print(f"creating new {data['name']}")
        return await model.create(data=data)
    return current_object


async def main() -> None:
    prisma = Prisma()
    data = Data()
    await prisma.connect()

    for ability_score in data.get_ability_scores():
        await upsert(
            prisma.abilityscore,
            {
                "name": ability_score["name"],
                "fullName": ability_score["full_name"],
                "desc": ability_score["desc"],
            },
        )
    print("finished ability scores")

    for alignment in data.get_alignments():
        await upsert(
            prisma.alignment,
            {
                "name": alignment["name"],
                "abbreviation": alignment["abbreviation"],
                "desc": alignment["desc"],
            },
        )
    print("finished alignments")

    for condition in data.get_conditions():
        await upsert(
            prisma.condition, {"name": condition["name"], "desc": condition["desc"]}
        )
    print("finished conditions")

    for damage_type in data.get_damage_types():
        await upsert(
            prisma.damagetype,
            {"name": damage_type["name"], "desc": damage_type["desc"]},
        )
    print("finished damage types")

    for equipment_category in data.get_equipment_categories():
        await upsert(prisma.equipmentcategory, {"name": equipment_category["name"]})
    print("finished equipment categories")

    for school in data.get_magic_schools():
        await upsert(prisma.school, {"name": school["name"], "desc": school["desc"]})
    print("finished schools")

    for rule in data.get_rules():
        rule_object = await upsert(
            prisma.rule, {"name": rule["name"], "desc": rule["desc"]}
        )
        subsection_names = [subsection["name"] for subsection in rule["subsections"]]
        for rule_section in data.get_rule_sections():
            if rule_section["name"] in subsection_names:
                await upsert(
                    prisma.rulesection,
                    {
                        "name": rule_section["name"],
                        "desc": rule_section["desc"],
                        "ruleId": rule_object.id,
                    },
                )
    print("finished rules and rule sections")

    for skill in data.get_skills():
        ability_score_object = await prisma.abilityscore.find_first(
            where={"name": {"equals": skill["ability_score"]["name"]}}
        )
        await upsert(
            prisma.skill,
            {
                "name": skill["name"],
                "desc": skill["desc"],
                "abilityScoreId": ability_score_object.id,
            },
        )
    print("finished skills")

    for weapon_property in data.get_weapon_properties():
        await upsert(
            prisma.weaponproperty,
            {
                "name": weapon_property["name"],
                "desc": weapon_property["desc"],
            },
        )
    print("finished weapon properties")

    await prisma.disconnect()


if __name__ == "__main__":
    asyncio.run(main())
