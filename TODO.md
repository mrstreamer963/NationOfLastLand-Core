1. Сделать интеграцию с UNITY
2. Продолжить доработки ядра - обработку hp и удаление waste

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

