// список типов ячеек в которых могут располагаться активные предметы

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActiveItemSlotType {
    LeftHand,
    RightHand,
    Head,
    Chest,
    Legs,
    Feet,
    VehicleSlot
}
