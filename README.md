# Сервер расброса дропа выпадающего из босса

## Точки http запросов

- api/v0/itemtypes
- api/v0/items{/:id}
- api/v0/bosses{/:id}

## Получение

http get [ip_addr:port]/[end_point]


## Заполнение

### Типы предметов
``` bash
http post [ip_addr:port]/api/v0/itemtypes _label=typename
```
### Предметы
``` bash
http post [ip_addr:port]/api/v0/items api/v0/items _label=item_name \
_type:=type_id _exchangable:=bool_exchange_flag \ 
_equals:=exchange_val
```
### Боссы
``` bash
http post [ip_addr:port]/api/v0/bosses _label=boss_name \
_level:=level_val _drop:='[{"_id": item_id, "_probability": probability_val},...]'
```
