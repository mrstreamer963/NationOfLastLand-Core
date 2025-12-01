1. Сделать интеграцию с UNITY
2. Продолжить доработки ядра - обработку hp и удаление waste
3. Переделать загрузку yml из статического включения в исходики в загрузку через http:
       dataInfo = download('data.yml), 
       core.load(dataInfo) 


hecs::serialize - реализовать, оказывается все уже есть :)

Базовые параметры:

Health,

Механика боя:

Лучше всего использовать трёхуровневую систему:
Уязвимость (vulnerability > 1.0 )
Урон усиливается (×1.5, ×2 и т.д.)
Сопротивление ( 0.0 < resistance < 1.0 ) Урон снижается (×0.5, ×0.2)
Иммунитет ( resistance = 0.0 ) Урон = 0
📌 Отсутствие записи ≠ иммунитет.
По умолчанию — урон проходит полностью (множитель = 1.0). 

pub enum DamageType {
    Physical,   // можно разбить на Slash, Pierce и т.д.
    Fire,
    Ice,
    Lightning,
    Poison,
    Holy,
    Magic,
}

Unit -> WeaponSlot<SlotType, Weapon>

Weapon -> attack 1
       -> attack 2
       -> attack 3

Attack_XXX: 
- distance
- toxic_power


Trash WeaponSlot<>


Waste -> WeaponSlot<Top, WasteWeapon>
                              WasteWeapon -> Attacks<WasteAttack>

WasteAttack: {
       type: AcidAttackType
       distance,
       power
}

Vehicle -> WeaponSlot<Top, CleanWeapon>
                            CleanWeapon -> Attacks<CleanAttack>

CleanAttack: {
       type: CleanAttackType
       distance,
       power
}

Vehicle -> ActiveItemSloth
              <VehicleSlot_1, Option(Item)>
              <VehicleSlot_2, Option(Item)>
              <VehicleSlot_3, Option(Item)>
           BackPack[
              Item, Item, Item
           ]




## по поводу автогенерации полей через макросы и derive -
и еще раз - desctiption не конвертится напрямую в entity - в нем содержатся только макс/мин параметры, например max_health: 100,
который при создании entity преобразуется в компонент Health { max: 100, min: 100 }
 
## по поводу автогенерации полей через макросы и derive -
нейросетка нагенерила множество ужасного кода, но в итоге все сводилось к генерации дефолтного бандла и кучи ручных insert тех полей, которые
в исходном отстутствовали - это pos, rot, speed, etc.... В итоге было принято решение создавать единый бандл на основе
данных из описания и отсутствующих полей - 

```
        if let Some(vehicle_data) = self.descriptions.vehicles.get(vehicle_key) {
            self.spawn_entity((
                pos,
                Rot { x: 0.0, y: 0.0 },
                vehicle_data.max_speed,
                Velocity { x: 0.0, y: 0.0 },
                vehicle_data.health,
                Force(100.0),
                IsWaitingTarget {},
                EntityType::Vehicle,
                Vehicle {},
            ));
```
