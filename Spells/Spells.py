from Spells.Spell import Spell


class Spells:
    def __init__(self) -> None:
        self.spells: dict[str, Spell] = {}

    def get_spells(self) -> dict[str, Spell]:
        return self.spells

    def get_spell(self, spell_name: str) -> Spell:
        return self.spells[spell_name]

    def add_spell(self, spell: Spell) -> None:
        self.spells[spell.name] = spell
