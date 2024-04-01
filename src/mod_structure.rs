pub mod structure {
    #[derive(Clone, Eq, PartialEq)]
    pub struct HintRule {
        pub directions: Vec<(isize, isize)>,
        pub times: usize,
    }

    #[derive(Clone)]
    pub struct ContextPosition {
        pub obstacle: TypeObstacle,
    }


    #[derive(Eq, PartialEq, Clone)]
    pub enum TypeObstacle {
        FREE,
        OBSTACLE,
        DEAD,
        TREASURE,
        HINT(HintRule),
    }

    impl HintRule {
        pub fn new(directions: Vec<(isize, isize)>, times: usize) -> Self {
            HintRule { directions, times }
        }
    }

    impl ContextPosition {
        pub fn new(obstacle: TypeObstacle) -> Self {
            ContextPosition { obstacle }
        }
    }
}
