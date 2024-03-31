pub mod structure {
    use rand::Rng;

    pub trait ObstacleRule {
        fn apply_move(&self) -> isize;
        fn random(&self, num: usize) -> isize {
            rand::thread_rng().gen_range(0..num + 1) as isize
        }
        fn clone_box(&self) -> Box<dyn ObstacleRule>;
    }

    pub struct ProcessMoveRule;

    pub struct RegressMoveRule;

    pub struct NoneRule;

    pub struct ContextPosition {
        pub obstacle: TypeObstacle,
        rule: Box<dyn ObstacleRule>,
    }

    #[derive(Clone)]
    pub struct PositionPlayer {
        pub current_position: (usize, usize),
        pub context: ContextPosition,
    }

    #[derive(Eq, PartialEq, Copy, Clone)]
    pub enum TypeObstacle {
        FREE,
        OBSTACLE,
        DEAD,
        TREASURE,
    }

    impl Clone for Box<dyn ObstacleRule> {
        fn clone(&self) -> Box<dyn ObstacleRule> {
            self.clone_box()
        }
    }


    impl PositionPlayer {
        pub fn new(current_position: (usize, usize), context: ContextPosition) -> Self {
            PositionPlayer {
                current_position,
                context,
            }
        }

        pub fn change_position(&mut self, position: (usize, usize),
                               context_position: ContextPosition) {
            self.current_position = position;
            self.context = context_position;
        }

        pub fn get_obstacle(&self) -> TypeObstacle {
            self.context.obstacle
        }
    }

    impl ContextPosition {
        pub fn new(obstacle: TypeObstacle, rule: Box<dyn ObstacleRule>) -> Self {
            ContextPosition { obstacle, rule }
        }
    }

    impl ObstacleRule for ProcessMoveRule {
        fn apply_move(&self) -> isize {
            self.random(5)
        }

        fn clone_box(&self) -> Box<dyn ObstacleRule> {
            Box::new(self.clone())
        }
    }

    impl Clone for ProcessMoveRule {
        fn clone(&self) -> Self {
            ProcessMoveRule {}
        }
    }

    impl Clone for RegressMoveRule {
        fn clone(&self) -> Self {
            RegressMoveRule {}
        }
    }

    impl Clone for ContextPosition {
        fn clone(&self) -> Self {
            ContextPosition {
                obstacle: self.obstacle,
                rule: self.rule.clone(),
            }
        }
    }

    impl ObstacleRule for RegressMoveRule {
        fn apply_move(&self) -> isize {
            -self.random(4)
        }

        fn clone_box(&self) -> Box<dyn ObstacleRule> {
            Box::new(self.clone())
        }
    }

    impl ObstacleRule for NoneRule {
        fn apply_move(&self) -> isize {
            0
        }

        fn clone_box(&self) -> Box<dyn ObstacleRule> {
            Box::new(self.clone())
        }
    }

    impl Clone for NoneRule {
        fn clone(&self) -> Self {
            NoneRule {}
        }
    }
}
