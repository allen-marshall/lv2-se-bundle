//! Constants representing various RDF class hierarchies and instances from the LV2 standard.

// TODO: Explore possibility of generating some of the information in this module automatically at
// build time, maybe using macros and/or RDF?

enum_set_type! {
    /// Identifiers for standard LV2 plugin classes.
    ///
    /// This type's implementations of [`Ord`](std::cmp::Ord) and
    /// [`PartialOrd`](std::cmp::PartialOrd) have little semantic meaning, and exist mainly for use
    /// with collections that require an ordered element type. In particular, superclasses are not
    /// guaranteed to have a particular ordering relative to their subclasses.
    pub enum PluginType {
        Delay,
        Reverb,
        Distortion,
        Waveshaper,
        Dynamics,
        Amplifier,
        Compressor,
        Envelope,
        Expander,
        Gate,
        Limiter,
        Filter,
        Allpass,
        Bandpass,
        Comb,
        EQ,
        MultiEQ,
        ParaEQ,
        Highpass,
        Lowpass,
        Generator,
        Constant,
        Instrument,
        Oscillator,
        Modulator,
        Chorus,
        Flanger,
        Phaser,
        Simulator,
        Spatial,
        Spectral,
        Pitch,
        Utility,
        Analyser,
        Converter,
        Function,
        Mixer
    }
}

enum_set_type! {
    /// Identifiers for standard host features that can be supported and/or required by a plugin or
    /// LV2 UI. Does not include UI port protocol features; for those, see
    /// [`UiPortProtocol`](self::UiPortProtocol).
    pub enum HostFeature {

        /// A plugin can support this feature to indicate that it is a 'hard real-time' plugin. Note
        /// that the LV2 standard defines some restrictions on the behavior of hard real-time
        /// plugins. Among other things, they must not perform heap allocations or I/O operations in
        /// certain functions.
        HardRtCapable,

        /// A plugin can require this feature to indicate that it will not work correctly if input
        /// and output ports are connected to the same memory location. This effectively disables
        /// in-place processing.
        InPlaceBroken,

        /// A plugin can require this feature to indicate that it must operate 'live', e.g. because
        /// it receives real-time input from a socket or other source besides the host.
        Live,

        /// Indicates that the host will specify (as LV2 options) upper and lower bounds on the
        /// number of samples it will use per block.
        BoundedBlockLength,

        /// A plugin can support this feature to indicate that it 'prefers' a constant/regular block
        /// length. For plugins that support this feature, hosts may want to avoid block splits that
        /// would normally be done for accuracy purposes.
        CoarseBlockLength,

        /// Indicates that the host will use a constant block length, specified using LV2 options.
        FixedBlockLength,

        /// Indicates that the host will always use a power-of-two block length.
        PowerOf2BlockLength,

        /// Indicates that the host can receive log messages from the plugin.
        Logging,

        /// Indicates that the host can provide LV2 options to the plugin.
        Options,

        /// Indicates that the host clamps values for ports that are marked as having strict bounds.
        StrictBounds,

        /// Indicates that the host can allow plugins to change the size of their output port
        /// buffers.
        ResizeBuffer,

        /// A plugin can support this feature to indicate that its bundle data contains a default
        /// LV2 state dictionary that should be loaded after initializing the plugin. If the host
        /// and plugin both support this feature, the host MUST load the default state after
        /// initializing the plugin.
        LoadDefaultState,

        /// Indicates that the host provides file/directory creation functionality for the plugin.
        MakePath,

        /// Indicates that the host provides mapping between actual file system paths and 'abstract'
        /// paths associated with the plugin. If the plugin stores file paths in its stored state,
        /// that stored state should use the abstract versions of the paths.
        MapPath,

        /// A plugin can support this feature to indicate that its state restore method is thread
        /// safe. This means the state restore can be safely called concurrently with the plugin's
        /// audio processing.
        ThreadSafeRestore,

        /// An LV2 UI can require this feature to indicate that its GUI should not be resizable by
        /// the user, *and* that the UI will not resize the GUI on its own.
        FixedGuiSize,

        /// An LV2 UI can support this feature to indicate that it provides an `idle` callback that
        /// should be called repeatedly to keep the UI up to date.
        IdleInterface,

        /// An LV2 UI can require this feature to indicate that the user should not be allowed to
        /// resize its GUI.
        NoUserResize,

        /// Indicates that the host can provide a 'parent' for an LV2 UI's GUI. (The parent may be a
        /// widget, canvas, etc.) The LV2 specification recommends against requiring this feature.
        GuiParent,

        /// Indicates that the host can map a port symbol to the corresponding port index.
        PortMap,

        /// Indicates that the host allows the LV2 UI to dynamically subscribe to updates from
        /// plugin ports.
        PortSubscribe,

        /// Indicates that the host can receive size change requests from the LV2 UI, and can be
        /// notified of the UI's current size.
        ResizeGui,

        /// Indicates that the host can receive notifications from the LV2 UI indicating which
        /// port's control is being touched by the user. Hosts may want to use this to disable
        /// automation for the currently touched port so the user can control it.
        UiTouch,

        /// Indicates that the host can allow plugins to create URID mappings.
        UridMap,

        /// Indicates that the host can allow plugins to remove previously created URID mappings.
        UridUnmap,

        /// Indicates that the host provides work scheduling for tasks that need to be executed
        /// outside the audio thread.
        WorkSchedule
    }
}

enum_set_type! {
    /// Identifiers for standard host features that represent LV2 port protocols used by LV2 UIs. A
    /// port protocol defines a way for the LV2 UI and plugin to communicate port values.
    pub enum UiPortProtocol {
        /// Port protocol for transferring atoms.
        Atom,

        /// Port protocol for transferring atoms from an atom event sequence. The event time stamps
        /// are not transferred.
        AtomEvent,

        /// Port protocol for transferring single floats.
        Float,

        /// Port protocol for transferring audio port peak measurements to the LV2 UI.
        Peak
    }
}

enum_set_type! {
    /// Identifiers for standard extension interfaces that a plugin or LV2 UI can provide.
    pub enum PluginExtensionData {
        /// Extension interface for dynamically setting and getting LV2 options.
        Options,

        /// Extension interface for saving and restoring plugin state.
        State,

        /// Extension interface providing an `idle` callback that should be called repeatedly to
        /// keep the LV2 UI up to date.
        IdleInterface,

        /// Extension interface allowing the host to request that an LV2 UI resize its GUI.
        ResizeGui,

        /// Extension interface allowing the host to request that an LV2 UI show or hide its GUI.
        ShowInterface,

        /// Extension interface for handling worker tasks that need to be executed outside the audio
        /// thread.
        Worker
    }
}

enum_set_type! {
    /// Identifiers for standard LV2 atom classes. Non-standard atom classes can exist but are not
    /// represented by this type.
    ///
    /// Note: This type's implementations of [`Ord`](std::cmp::Ord) and
    /// [`PartialOrd`](std::cmp::PartialOrd) have little semantic meaning, and exist mainly for use
    /// with collections that require an ordered element type. In particular, superclasses are not
    /// guaranteed to have a particular ordering relative to their subclasses.
    pub enum AtomType {
        /// Base class for the atom class hierarchy.
        Atom,

        /// Boolean atom type.
        Bool,

        /// Type for atoms that provide a generic chunk of memory, with size determined by the
        /// atom's size field.
        Chunk,

        /// Similar to an RDF literal. The atom contains UTF-8 data, with an optional language or
        /// type tag.
        Literal,

        /// Base class for numeric atom types.
        Number,

        /// Double-precision floating point number (always 64 bits).
        Double,

        /// Single-precision floating point number (always 32 bits).
        Float,

        /// Signed integer (always 32 bits).
        Int,

        /// Signed integer (always 64 bits).
        Long,

        /// Object atom type. Object atoms are essentially dictionaries with LV2 URIDs as keys and
        /// atoms as values.
        Object,

        /// Atom type representing a property of an [`Object`](self::AtomType::Object). An atom
        /// of this type contains a key-value pair.
        Property,

        /// An atom of this type contains a list of time-stamped atoms, which must all have the same
        /// pre-specified type.
        Sequence,

        /// UTF-8 string type.
        String,

        /// UTF-8 URI type.
        Uri,

        /// UTF-8 URI type with only a path component.
        Path,

        /// An atom of this type contains a list of atoms, which may have different types.
        Tuple,

        /// LV2 URID atom. A URID is a 32-bit unsigned integer that has been mapped to a URI.
        Urid,

        /// An atom of this type contains a list of atoms, which must all have the same
        /// pre-specified type.
        Vector,

        /// An atom of this type contains a list of [`Float`](self::AtomType::Float) atoms.
        Sound,

        /// An atom of this type contains a single MIDI event.
        MidiEvent,

        /// A specialized MIDI atom type.
        MidiSystemMessage,

        /// A specialized MIDI atom type.
        MidiSystemCommon,

        /// A specialized MIDI atom type.
        MidiQuarterFrame,

        /// A specialized MIDI atom type.
        MidiSongPosition,

        /// A specialized MIDI atom type.
        MidiSongSelect,

        /// A specialized MIDI atom type.
        MidiTuneRequest,

        /// A specialized MIDI atom type.
        MidiSystemExclusive,

        /// A specialized MIDI atom type.
        MidiSystemRealtime,

        /// A specialized MIDI atom type.
        MidiActiveSense,

        /// A specialized MIDI atom type.
        MidiClock,

        /// A specialized MIDI atom type.
        MidiContinue,

        /// A specialized MIDI atom type.
        MidiReset,

        /// A specialized MIDI atom type.
        MidiStart,

        /// A specialized MIDI atom type.
        MidiStop,

        /// A specialized MIDI atom type.
        MidiVoiceMessage,

        /// A specialized MIDI atom type.
        MidiAftertouch,

        /// A specialized MIDI atom type.
        MidiBender,

        /// A specialized MIDI atom type.
        MidiChannelPressure,

        /// A specialized MIDI atom type.
        MidiController,

        /// A specialized MIDI atom type.
        MidiNoteOff,

        /// A specialized MIDI atom type.
        MidiNoteOn,

        /// A specialized MIDI atom type.
        MidiProgramChange
    }
}

enum_set_type! {
    /// Identifiers for measurement units defined by the LV2 standard.
    pub enum Unit {
        Bar,
        Beat,

        /// Beats per minute.
        Bpm,

        Cent,
        Centimeter,
        Coefficient,
        Decibel,
        Degree,
        AudioFrame,
        Hertz,
        Inch,
        Kilohertz,
        Kilometer,
        Meter,
        Megahertz,
        MidiNote,
        Mile,
        Minute,
        Millimeter,
        Millisecond,
        Octave,
        Percent,
        Second,

        /// Semitone using 12-tone equal temperament.
        Semitone12Tet
    }
}

enum_set_type! {
    /// Identifiers for standard LV2 port properties that can apply to a port.
    pub enum PortProperty {
        /// Indicates that connecting the port to a non-null buffer is optional.
        ConnOptional,

        /// Indicates that the port's only valid values are those defined by its scale points.
        Enumeration,

        /// Indicates that the port's only valid values are integers.
        IntOnly,

        /// Indicates that the port is a 'sidechain' rather than a main port. Hosts are not required
        /// to connect anything to a sidechain port, not even a null pointer.
        SideChain,

        /// Indicates that the port reports the plugin's latency in samples.
        ReportsLatency,

        /// Indicates that the port's bounds (e.g. maximum and minimum) should be interpreted as
        /// multiples of the sample rate.
        BoundsRelativeToSampleRate,

        /// Indicates that the port's value represents a boolean. Ports with this flag should
        /// interpret positive values as true, and zero or negative values as false.
        Toggle,

        /// Indicates that changing the port's input value may cause audio artifacts.
        ChangeCausesArtifacts,

        /// Indicates that the port's signal should be interpreted as a smooth modulation signal.
        ContinuousCV,

        /// Indicates that the port's signal should be interpreted as a discrete modulation signal.
        DiscreteCV,

        /// Indicates that changing the port's input value may trigger expensive computation.
        ChangeExpensive,

        /// Indicates that the port's bounds (e.g. maximum and minimum) should be considered strict.
        StrictBounds,

        /// Indicates that the port's value is on a logarithmic scale.
        Logarithmic,

        /// Indicates that the port is not intended to receive a modulation/automation signal.
        NotAutomatic,

        /// Indicates that the port is not intended to be shown as a control in the GUI.
        NotOnGui,

        /// Indicates that the port represents a trigger, and should be reset to its default value
        /// when not being triggered.
        Trigger
    }
}

enum_set_type! {
    /// Identifiers for standard LV2 'designations' that can apply to a port. Does not include
    /// channel designations; for those, see [`PortChannel`](self::PortChannel).
    pub enum PortDesignation {
        /// Designates the port as representing an amplitude.
        Amplitude,

        /// Designates the port as representing an envelope's attack duration.
        Attack,

        /// Designates the port as a boolean bypass channel. A value of true means bypassed.
        Bypass,

        /// Designates the port as representing a cutoff frequency.
        CutoffFrequency,

        /// Designates the port as representing an envelope's decay duration.
        Decay,

        /// Designates the port as representing an envelope's delay duration.
        Delay,

        /// Designates the port as representing a dry level for a signal.
        DryLevel,

        /// Designates the port as representing a frequency.
        Frequency,

        /// Designates the port as representing a gain in decibels.
        Gain,

        /// Designates the port as representing an envelope's hold duration.
        Hold,

        /// Designates the port as representing a rectangular wave's pulse width.
        PulseWidth,

        /// Designates the port as representing a compression ratio.
        CompressionRatio,

        /// Designates the port as representing an envelope's release duration.
        Release,

        /// Designates the port as representing a filter resonance.
        Resonance,

        /// Designates the port as representing a sample rate in Hertz.
        SampleRate,

        /// Designates the port as representing an envelope's sustain level.
        Sustain,

        /// Designates the port as representing a compression threshold.
        CompressionThreshold,

        /// Designates the port as representing a waveform.
        Waveform,

        /// Designates the port as representing a wet/dry ratio.
        WetDryRatio,

        /// Designates the port as representing a wet level for a signal.
        WetLevel
    }
}

enum_set_type! {
    /// Identifiers for standard LV2 channel designations that can apply to a port.
    pub enum PortChannel {
        /// Designates the port as a main control channel. Typically used for a MIDI channel that
        /// controls an instrument plugin.
        Control,

        Center,
        CenterLeft,
        CenterRight,
        Left,

        /// Designates the port as a low-frequency effects channel.
        LowFrequencyEffects,

        RearCenter,
        RearLeft,
        RearRight,
        Right,
        Side,
        SideLeft,
        SideRight
    }
}