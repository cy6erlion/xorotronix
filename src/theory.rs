/// ⚛️ Foundations upon which everything is built.
pub mod physics {
    /// ➕ Protons reside in the nucleus of an atom. Protons have a
    /// positive electrical charge.
    /// Every element has a specific number of protons, the element's
    /// __atomic number__.
    pub struct Proton;

    /// ➖ Electrons reside in the nucleus of an atom. Electrons
    /// have a negative electric charge.
    pub struct Electron;

    /// ⚫ The __nucleus__ of an atom, containing __protons__ and
    /// __electrons__. Protons have a positive electrical charge,
    /// electrons a negative charge, and neutrons have no electrical
    /// charge.
    pub struct Nucleus {
        pub protons: Vec<Proton>,
        pub electrons: Vec<Electron>,
    }

    /// ⚛️ The __atom__ is the primary building block of matter and
    /// is made up of a __nucleus__, containing __protons__ and
    /// __electrons__.
    ///
    /// Unless modified by chemical, mechanical, or electrical
    /// processes, all atoms are electrically neutral because they
    /// have the same number of electrons as protons.
    /// If an atom loses electrons, is has more protons than electrons
    ///  and thus has a net positive charge. If an atom gains
    /// electrons, it has more electrons than protons and a net
    /// negative charge.
    ///
    /// Atoms or molecules with a positive or negative charge are
    /// called __ions__.
    /// Electrons not bound to any atom, or __free electrons__, can be
    /// also considered as ions because they have a negative charge.
    pub struct Atom {
        pub nucleus: Nucleus,
    }

    /// 🧩 An __element__ (or chemical element) is a type of atom
    /// that has a specific number of protons, the element's
    /// __atomic number__. Each different element such as iron,
    /// oxygen, silicon or bromine has a distinct chemical and
    /// physical identity determined primarily by the number of
    /// protons.
    pub struct Element {
        pub protons: Vec<Proton>,
        pub electrons: Vec<Electron>,
    }

    /// 💠 A __molecule__ is two or more atoms bonded together and
    /// acting a single particle.
    pub struct Molecule {
        pub bonding: Vec<Atom>,
    }
}

/// ⚡ Any peice of matter that has a net positve or negative
/// electrical charge is said to be __electrically charged__. An
/// electrical force exists between electrically charged particles,
/// pushing charges of the same type apart (like charges repel each
/// other) and pulling opposite charges together (opposite charges
/// attract).
///
/// Moving charges in a magnetic field also generate an electrical
/// force. This is the __electromotive force__ (or EMF), the source
/// of energy that causes charged particles to move.
///
/// Under most conditions, the number of positive and negative charges
/// in any volume of space is very close to balanced and so the region
/// has no net charge. When there are extra positive ions in one
/// region and extra negative ions (or electrons) in another region,
/// the resulting EMF attracts the charges toward each other. The
/// direction of the force, from the positive region to the negative
/// region, called its __polarity__. Because an imbalance of charge
/// between two regions generates an EMF, its voltage is always
/// measured between two points.
pub mod electric {
    /// 🌀 __Voltage__ is the general term for the strength of the
    /// electromotive force or the difference in electrical potential
    /// between two points. Voltage and EMF are sometimes used
    /// interchangeably.
    ///
    /// Voltage difference can be created in a variety of ways. For
    /// example, chemical ions can be physically separated to form a
    /// battery. The resulting charge imbalance creates a voltage
    /// difference at the battery terminals so that if a conductor is
    /// connected to both terminals at once electrons flow between the
    /// terminals and gradually eliminate the charge imbalance,
    /// discharging the battery's stored energy. Mechanical means such
    /// as friction (static electricity, lightning) and moving
    /// conductors in a magnetic field (generators) can also produce
    /// voltages. Devices or systems that produce voltage are called
    /// __voltage sources__.
    pub struct Voltage;

    ///🚿 If there is no path by which electric charge can move in
    /// response to an EMF (called a conducting path), the charges
    /// cannot move together and so remain separated. If a conducting
    /// path is available, then the electrons or ions will flow along
    /// the path, neutralizing the net imbalance of charge. The
    /// movement of electrical charge is called __electric current__.
    /// Materials through which current flows easily are called
    /// __conductors__. Most metals, such as copper, or aluminum are
    /// good conductors. Materials in which it is difficult for
    /// current to flow are __insulators__. __Semiconductors__, such as
    /// silicon or germanium, are materials with much poorer
    /// conductivity than metals. Semiconductors can be chemically
    /// altered to acquire properties that make them useful in
    /// solid-state devices such as diodes, transistors and
    /// integrated circuits.
    ///
    /// Electrons move in the direction of positive voltage -- that is
    /// __electronic current__. __Conventional current__ takes the
    /// other point of view -- of positive charges moving in the
    /// direction of negative voltage. Conventional current was the
    /// original model for electricity and results from an arbitrary
    /// decision made by Benjamin Franklin in the 18th century when
    /// the nature of electricity and atoms was still unknown. It can
    /// be imagined as electrons flowing "backward" and is completely
    /// equivalent to electronic current.
    /// Conventional current is used in nearly all electronic
    /// literature.
    pub struct Current;

    /// 📿 A __circuit__ is any conducting path through wich current can
    /// flow between two points that have different voltages. An __open
    /// circuit__ is a circuit in which a desired conducting path is
    /// interrupted, such as by a broken wire or a switch.
    /// A __short circuit__ is a circuit in which a conducting path allows
    /// current to flow directly between the two points at different
    /// voltages.
    ///
    /// The two fundamental types of circuits are shown bellow:
    ///
    /// ```text
    ///                       ___
    ///            .---------|___|----------.
    ///            |                        |
    ///            |                        |
    ///            |                        |
    ///            o                        |
    ///           +|                        |
    ///           ---                       |
    ///            -                       .-.
    ///           -|                       | |
    ///            |                       | |
    ///            o                       '-'
    ///            |                        |
    ///            |                        |
    ///            |                        |
    ///            |         ___            |
    ///            '--------|___|-----------'
    ///
    ///               (A) Series Circuit
    ///
    /// ```
    ///
    /// Circuit __A__ shows a __series circuit__ in which there is only
    /// one current path. The current in this circuit flows from the
    /// voltage source's positive terminal, through the three
    /// __resistors__ and back to the battery's negative terminal. Current
    /// is the same at every point in the a series circuit.
    ///
    /// ```text
    ///          .---------------------.
    ///          |                     |
    ///          |                     |
    ///          |/--------.           |
    ///        + o         |           |
    ///         ---       .-.         .-.
    ///          -        | |         | |
    ///        - |        | |         | |
    ///          o        '-'         '-'
    ///          |\--------'           |
    ///          |                     |
    ///          |                     |
    ///          |                     |
    ///          '---------------------'
    ///           (B) Parallel Circuit
    /// ````
    ///
    /// Circuit __B__ shows a __parallel circuit__ in which there are
    /// multiple paths for the current to take. One terminal of both
    /// resistors is connected to the battery;s positive terminal. The
    /// other terminal of both resistors is connected to the battery's
    /// negative terminal. Current flowing out of the battery's positive
    /// terminal divides into smaller currents that flow through the
    /// the individual resistors and then recombine at the battery's
    /// negative terminal. All components in a parallel circuit experience
    /// the same voltage.
    ///
    /// __All circuits are made up of series and parallel combinations of
    /// components and sources of voltage and current.__
    ///
    /// ## Branch
    ///
    /// Represents a single element / any two terminal device such
    /// as a voltage source, current source or a resistor.
    ///
    /// ```text
    ///                o
    ///                |
    ///                |
    ///               .-.
    ///              |   |
    ///              |   |
    ///               '-'
    ///                |
    ///                |
    ///                o
    ///```
    ///
    /// ## Node
    ///
    /// Is the point of connection between two or more branches.
    ///
    /// ```terminal
    ///              ___    Node    ___
    ///            -|___|----o-----|___|-
    ///                      |
    ///                      |
    ///                      |
    ///                     .-.
    ///                     | |
    ///                     | |
    ///                     '-'
    ///                      |
    ///```
    ///
    /// # Loop
    ///
    /// Any closed path in a circuit
    ///
    /// ```terminal
    ///          Node 1        Node 1      Node 1
    ///             o------------o-----------o
    ///             |            |           |
    ///            .-.          .-.         .-.
    ///            | |  Loop 1  | |  Loop 2 | |
    ///            | |          | |         | |
    ///            '-'          '-'         '-'
    ///             |            |           |
    ///             o------------o-----------o
    ///           Node 2        Node 2      Node 2
    ///```
    pub struct Circuits;

    /// ➖ Direct Current or dc is the flow of current in only one
    /// direction.
    pub struct DirectCurrent;

    /// ➖ Alternating current or ac is current that changes
    /// direction, In an ac circuit, both the current and the voltage
    /// reverse direction. For nearly all ac signals in electronics
    /// and radio, the reversal is __periodic__, meaning that the
    /// change in direction occurs on a regular basis. The rate of
    /// reversal may range from a few times per second to many billion
    /// times per second.
    pub struct AlternatingCurrent;

    /// 🛡️ Any conductor connected to points at different voltage will
    /// allow current to pass between the points. No conductor is
    /// perfect or lossless, however, at least not at normal
    /// temparatures. The moving electrons collide with the atoms
    /// making up the conductor and lose some of their energy by
    /// causing the atoms to vibrate, which is observed externally as
    /// heat. The property of energy loss due to interactions between
    /// moving charges and atoms of the conductor is called
    /// __resistance__. The amount of resistance to current is
    /// measured in __ohms__ (Ω) and is represented by __r__ or __R__
    /// in equations.
    ///
    /// Suppose we have two conductors of the same size and shape, but
    /// of different materials. Because all materials have different
    /// internal structures, the amount of energy lost by current
    /// flowing through the material is also different. The material's
    /// ability to impede current flow is its __resistivity__.
    /// Numerically, the resistivity of a material is given by the
    /// resistance, in ohms, of a cube of the material measuring one
    /// centimeter on each edge.The symbol for resistivity is the
    /// Greek letter rho, ρ.
    ///
    /// The longer a conductor's physical path, the higher the
    /// resistance of that conductor. For direct current and
    /// low-frequency alternating current (up to a few thousand hertz)
    /// the conductor's resistance is inversely proportional to the
    /// cross-sectional area of the conductor. Given two conductors
    /// of the same material and having the same length, but differing
    /// in cross-sectional area, the one with the larger area (for
    /// example, a thicker wire or sheet) will have the lower
    /// resistance.
    ///
    /// One of the best conductors is copper, and it is frequently
    /// convenient to compare the resistance of a material under
    /// consideration with that of a copper conductor of the same
    /// size and shape.
    ///
    /// ## Relative Resistivity of Metals
    ///
    /// | Material                  | Resistivity Compared to Copper |
    /// |---------------------------|--------------------------------|
    /// | Aluminum (pure)           | 1.60                           |
    /// | Brass                     | 3.7-4.90                       |
    /// | Cadmium                   | 4.40                           |
    /// | Chromium                  | 8.10                           |
    /// | Copper (hard-drawn)       | 1.03                           |
    /// | Copper (annealed)         | 1.00                           |
    /// | Gold                      | 1.40                           |
    /// | Iron (pure)               | 5.68                           |
    /// | Lead                      | 12.80                          |
    /// | Nickel                    | 5.10                           |
    /// | Phosphor bronze           | 2.8-5.40                       |
    /// | Silver                    | 0.94                           |
    /// | Steel                     | 7.6-12.70                      |
    /// | Tin                       | 6.70                           |
    /// | Zinc                      | 3.40                           |
    pub struct Resistance;

    /// 🖇️ The reciprocal of resistance (I/R) is __conductance__. It
    /// is usually represented by the symbol G. A circuit having high
    /// conductance has low resistance, and vice versa. In radio
    /// work, the term is used chiefly in connection with
    /// electron-tube and field-effect transistor characteristics.
    /// The units of conductance are siemens (S). A resistance of 1
    /// Ω has a conductance of 1 S, a resistance of 1000 Ω has a
    /// conductance of 0.001 S, and so on. A unit frequently used in
    /// regard to vacuum tubes and field-effect transistor is µS or
    /// one millionth of a siemens. It is the conductance of a 1-MΩ
    /// resistance. Siemens have replaced the obsolete unit mho.
    pub struct Conductance;

    /// 🪫 Electrical components
    pub mod components {
        /// A package of material exhibiting a certain amount of
        /// resistance and made into a single unit or component is
        /// called a __resistor__. Next to the transistors built into
        /// microprocessors by the billion, resistors are the most
        /// common electronic component of all.
        pub struct Resistor;
    }

    pub mod laws {
        /// The amount of current that will flow through a conductor when
        /// a given voltage is applied will vary with the resistance of
        /// the conductor. __The lower the resistance, the greater the
        /// the current for given EMF__.
        ///
        /// __One ohm (Ω) is defined  as the amount of resistance
        /// that allows one ampere of current to flow between two points
        /// that have a potential difference of one volt.__ This
        /// proportional relationship is known as Ohm's Law:
        ///
        /// R = E / I
        ///
        /// where
        ///
        /// - R = resistance in ohms
        /// - E = voltage of EMF in volts
        /// - I = current in amperes
        ///
        /// Rearranging the equation gives the other common forms of Ohm's
        /// Law as:
        ///
        /// E = I * R
        ///
        /// and
        ///
        /// I = E / R
        ///
        /// Remeber that the quantities are in volts, ohms and amperes;
        /// other units cannot be used in the equations without first
        /// being converted. For example, if the current is in
        /// milliamperes you must first change it to the equivalent
        /// fraction of an ampere before substituting the value into the
        /// equations.
        pub struct OhmsLaw;

        /// Kirchoff's Current Law (KCL) states, __"The sum of all
        /// currents flowing into a node must balance the sum of all
        /// currents flowing out of a node"__.
        ///
        /// It follows that for a series circuit the current is the
        /// same everywhere.
        pub struct KirchhoffCurrentLaw;

        /// Things hooked in parrallel have the same voltage across
        /// them. Another way to say it is that the sum of the voltage
        /// drops around any closed circuit is zero. This is
        /// Kirchhoff's voltage law (KVL)
        pub struct KirchhoffVoltageLaw;
    }
}

/// 📏 Electrical Units and Their Namesakes
pub enum Units {
    /// __A__ measures __current__, coulombs per second. Named after
    /// Andree Ampere 1775 - 1836
    Ampere,
    /// __C__ measures __charge__. Named after Charles Coulomb
    /// 1736 - 1806.
    Coulomb,
    /// __F__ measures __capacitance__, coulombs per volt. Named after
    /// Michael Faraday 1791 - 1867
    Farad,
    /// __H__ measures __inductance__, volts per amp per second. Named
    /// after Joseph Henry 1797 - 1878
    Henry,
    /// __Hz__ measures __frequency__, cycles per second. Named after
    /// Heinrich Hertz 1857 - 1894.
    Hertz,
    /// __Ω__ measures resistance, volts per amp. Named after George
    /// Simon Ohm 1787 - 1854.
    Ohm,
    /// __W__ measures power, joules per second. Named after James
    /// Watt 1736 - 1819.
    Watt,
    /// __V__ measures voltage, joules per coulomb. Named after
    /// Alessandro Volta 1745 - 1827.
    Volt,
}
