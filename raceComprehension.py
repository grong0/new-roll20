import json

with open("./races.json") as json_object:
    data: list[dict] = json.load(json_object)

    race_urls: list[str] = data[0]['race_urls']
    finished_races: list[str] = data[1]['finished_races']

    for url in finished_races:
        race_urls.remove(url)
    
    print(race_urls)
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    