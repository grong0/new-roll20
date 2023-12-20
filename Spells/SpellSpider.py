from typing import NoReturn
from scrapy import Spider, Request  # type: ignore
from scrapy.http import Response  # type: ignore
from scrapy.crawler import CrawlerProcess  # type: ignore

from Spell import Spell  # type: ignore


class SpellsSpider(Spider):
    name = "SpellSpider"
    start_urls = ["http://dnd5e.wikidot.com/spells"]
    spell_urls = []
    new_loop = False

    def parse(self, response: Response):
        if self.new_loop == False:
            for products in response.css("tr"):  # type: ignore
                if products.css("th::text").get() != "Spell Name":
                    self.spell_urls.append(
                        "http://dnd5e.wikidot.com"
                        + str(products.css("a::attr(href)").get())
                    )
            self.new_loop = True

        else:
            spell = Spell(response)
            yield spell.get_as_dict()

        for x in self.spell_urls:
            yield (Request(x, callback=self.parse))


SpellsProcess = CrawlerProcess(
    settings={"FEEDS": {"spells.json": {"format": "json", "overwrite": True}}}
)
SpellsProcess.crawl(SpellsSpider)
SpellsProcess.start()  # the script will block here until all crawling jobs are finished
