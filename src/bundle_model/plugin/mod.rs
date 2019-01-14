/// Representation of LV2 plugins.

enum_set_type! {
    pub enum StdHostFeature {
        HardRtCapable,
        InPlaceBroken,
        Live,
        BoundedBlockLength,
        CoarseBlockLength,
        FixedBlockLength,
        PowerOf2BlockLength,
        Logging,
        Options,
        ResizeBuffer,
        LoadDefaultState,
        MakePath,
        MapPath,
        ThreadSafeRestore,
        FixedGuiSize,
        IdleInterface,
        NoUserResize,
        GuiParent,
        PortMap,
        PortSubscribe,
        ResizeGui,
        UiTouch,
        UridMap,
        UridUnmap,
        WorkSchedule
    }
}

enum_set_type! {
    pub enum StdPluginExtensionData {
        Options,
        State,
        IdleInterface,
        ResizeGui,
        ShowInterface,
        Worker
    }
}