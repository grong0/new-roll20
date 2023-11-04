import json
import asyncio
from prisma import Prisma

SPELLS_PATH = "./spells.json"

def get_object(filepath) -> dict:
    with open(filepath) as file_object:
        return json.load(file_object)

spell: dict = get_object(SPELLS_PATH)[0]

components1 = "V, S, M (a lead-based ink worth at least 10 gp, which the spell consumes)"
components2 = "V, S"
print(components1[components1.find("(")+1:components1.find(")")])
print(f"'{components1[:components1.find(' (')]}'")
print(components2[components2.find("("):components2.find(")")])

# async def main() -> None:
#     prisma = Prisma()
#     await prisma.connect()
    
#     # write queries here
#     user = await prisma.spell.upsert(
#         data={
#             'name': spell["name"],
#             'source': spell["source"],
            
#         }
#     )