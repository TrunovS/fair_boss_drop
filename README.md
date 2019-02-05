
# Сервер распределения дропа выпадающего из босса

## Точки http запросов

- api/v0/itemtypes
- api/v0/items{/:id}
- api/v0/bosses{/:id}

## Получение

http get [ip_addr:port]/[end_point]


## Заполнение

### Типы предметов
``` bash
http post [ip_addr:port]/api/v0/itemtypes label=typename
```
### Предметы
``` bash
http post [ip_addr:port]/api/v0/items label=item_name \
type:=type_id exchangable:=bool_exchange_flag \ 
equals:=exchange_val
```
### Боссы
``` bash
http post [ip_addr:port]/api/v0/bosses label=boss_name \
level:=level_val drop:='[{"id": item_id, "probability": probability_val},...]'
```
