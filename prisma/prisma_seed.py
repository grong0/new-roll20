import asyncio
from prisma import Prisma
import requests


class Data:
    damage_types: list[dict[str, str | list[str]]]

    def __init__(self) -> None:
        self.damage_types = []
        
        core_url = "https://www.dnd5eapi.co"
        response = requests.get(core_url + "/api/").json()

        damage_types_link = response["damage-types"]
        damage_types_response = requests.get(core_url + damage_types_link).json()
        for result in damage_types_response["results"]:
            damage_type = requests.get(core_url + result["url"]).json()
            name = damage_type["name"]
            desc = damage_type["desc"]
            self.damage_types.append({"name": name, "desc": desc})

    def get_damage_types(self) -> list[dict[str, str | list[str]]]:
        return self.damage_types


async def main() -> None:
    prisma = Prisma()
    data = Data()
    await prisma.connect()

    for damage_type in data.get_damage_types():
        await prisma.damagetype.create(data=damage_type)

    await prisma.disconnect()


if __name__ == "__main__":
    asyncio.run(main())
