// TODO: consider being immune to desease
// TODO: bacteriocarrier with some chance to become
// TODO: check desease status
// TODO: multiple infections
// TODO: speed of individual setter (we can change it for decrease)
// TODO: consider mutation for infection dat

pub mod individual {
    extern crate rand;
    use rand::Rng;

    #[test]
    #[should_panic(expected =
         "incubation_period cannot be bigger than desease_duration")]
    fn test_init_inf_constraints() {
        InfectionData::new(1, 1.0, 2, 1);
    }

    #[derive(Copy, Clone)]
    #[derive(Debug)]
    pub struct InfectionData {
        pub desease_duration:  u32,
        pub contagious_ind:    f32,
        pub incubation_period: u32,
        pub contagious_range:  u32,
    }

    impl InfectionData {
        pub fn new(
            desease_duration:  u32,
            contagious_ind:    f32,
            incubation_period: u32,
            contagious_range:  u32,
        ) -> InfectionData
        {
            assert!((incubation_period <= desease_duration), 
                    "incubation_period cannot be bigger than desease_duration");
            return InfectionData {
                desease_duration:  desease_duration,
                contagious_ind:    contagious_ind,
                incubation_period: incubation_period,
                contagious_range:  contagious_range,
            }
        }
    }

    #[test]
    #[should_panic(expected =
         "deasease_day cannot be longer desease_duration")]
    fn test_init_ind_constraints() {
        let inf_data = InfectionData::new(
            1, 0.8, 1, 2,
        );

        Individual::new(
            0, 0, 2, Some(7),
            100, 100, inf_data,
        );
    }

    #[test]
    fn test_contagious_init() {
        let inf_data = InfectionData::new(
            2, 0.8, 1, 2,
        );

        let new_ind = Individual::new(
            0, 0, 2, Some(2),
            100, 100, inf_data,
        );

        assert_eq!(new_ind.contagious, true);

        let new_ind = Individual::new(
            0, 0, 2, Some(1),
            100, 100, inf_data,
        );

        assert_eq!(new_ind.contagious, false);
    }

    #[test]
    fn test_walk_diff() {
        let inf_data = InfectionData::new(
            2, 0.8, 1, 2,
        );

        let mut new_ind = Individual::new(
            0, 0, 2, Some(2),
            100, 100, inf_data,
        );
        new_ind.walk((-1, -1));
        assert!(new_ind.x == 99 && new_ind.y == 99);

        let mut new_ind = Individual::new(
            0, 0, 2, Some(2),
            100, 100, inf_data,
        );
        new_ind.walk((101, 101));
        assert!(new_ind.x == 1 && new_ind.y == 1);

        new_ind.walk((49, 49));
        println!("{:?}", new_ind);
        assert!(new_ind.x == 50 && new_ind.y == 50);

        new_ind.walk((150, 150));
        assert!(new_ind.x == 0 && new_ind.y == 0);

        new_ind.walk((-200, -200));
        assert!(new_ind.x == 0 && new_ind.y == 0);
    }

    #[test]
    fn test_infected() {
        let inf_data = InfectionData::new(
            2, 0.8, 1, 2,
        );

        let mut new_ind = Individual::new(
            0, 0, 2, None,
            100, 100, inf_data,
        );

        new_ind.make_infected();
        assert_eq!(0, new_ind.desease_day.unwrap());

        new_ind.desease_day = Some(2);
        new_ind.make_infected();
        assert_eq!(2, new_ind.desease_day.unwrap());
    }

    #[test]
    fn test_inf_progress() {
        let inf_data = InfectionData::new(
            2, 0.8, 1, 2,
        );

        let mut new_ind = Individual::new(
            0, 0, 2, None,
            100, 100, inf_data,
        );
        new_ind.make_infected();

        new_ind.develop_inf();
        assert_eq!(new_ind.desease_day.unwrap(), 1);
        assert_eq!(new_ind.contagious, false);

        new_ind.develop_inf();
        assert_eq!(new_ind.desease_day.unwrap(), 2);
        assert_eq!(new_ind.contagious, true);

        new_ind.develop_inf();
        assert_eq!(new_ind.desease_day.is_none(), true);
        assert_eq!(new_ind.contagious, false);
    }

    #[derive(Debug)]
    pub struct Individual {
        // x, y - define position of individual on map
        // desease_day - we can use this to see if individual
        // is contag.
        // speed - how much steps he can make in one turn
        x: u32,
        y: u32,
        max_x: i32,
        max_y: i32,
        speed: u32,

        desease_day: Option<u32>,
        contagious: bool,
        inf_data: InfectionData,
    }

    impl Individual {
        pub fn new(
                x: u32, y: u32,
                speed: u32,
                desease_day: Option<u32>,
                max_x: i32, max_y: i32,
                inf_data: InfectionData,
        ) -> Individual 
        {
            // need to make sure that runtime passed value
            // is not bigger than possible duration of desease
            match desease_day {
                Some(x) => {
                    assert!((inf_data.desease_duration >= x), 
                        "deasease_day cannot be longer desease_duration");
                },
                None => {},
            }

            let mut contagious: bool = false;
            match desease_day {
                Some(x) => {
                    if x > inf_data.incubation_period {
                        contagious = true
                    }
                },
                None => {}
            }
            
            Individual {
                x: x,
                y: y,
                desease_day: desease_day,
                speed: speed,
                max_x: max_x, max_y: max_y,
                inf_data: inf_data,
                contagious: contagious,
            }
        }

        #[allow(dead_code)]
        pub fn get_position(&self) -> (u32, u32) {
            (self.x, self.y)
        }

        #[allow(dead_code)]
        pub fn get_status(&self) -> bool {
            self.contagious
        }

        // #[allow(dead_code)]
        fn walk(&mut self, diff: (i32, i32)) {
            // should be able to move individual
            // on the field
            let (x_diff, y_diff) = diff;
            let new_x_pos = (self.x as i32 + x_diff) % self.max_x;
            let new_y_pos = (self.y as i32 + y_diff) % self.max_y;

            if new_x_pos < 0 {
                self.x = (self.max_x + new_x_pos ) as u32;
            } else {
                self.x = new_x_pos as u32;
            }

            if new_y_pos < 0 {
                self.y = (self.max_y + new_y_pos) as u32;
            } else {
                self.y = new_y_pos as u32;
            }
        }

        fn generate_move(&self) -> (i32, i32) {
            let mut rng = rand::thread_rng();
            let range = (1 * self.speed, -1 * self.speed as i32);
            let x_diff = rng.gen_range(range.0 as i32, range.1 as i32);
            let y_diff = rng.gen_range(range.0 as i32, range.1 as i32);
            return (x_diff, y_diff);
        }

        #[allow(dead_code)]
        pub fn make_infected(&mut self) {
            // if individual already infected - 
            // we can ignore this
            match self.desease_day {
                Some(_) => {},
                None => self.desease_day = Some(0),
            }
        }

        fn develop_inf(&mut self) {
            // if individual is sick - 
            // desease progress each turn
            match self.desease_day {
                Some(mut x) => {
                    x += 1;
                    self.desease_day = Some(x);
                    if x > self.inf_data.incubation_period {
                        self.contagious = true;
                    }
                    if x > self.inf_data.desease_duration {
                        self.contagious = false;
                        self.desease_day = None;
                    }
                },
                None => {},
            }
        }

        #[allow(dead_code)]
        pub fn make_turn(&mut self) {
            
            self.walk(self.generate_move());
            self.develop_inf();

        }
    }
}

pub mod individual_group {
    use super::individual::{Individual, InfectionData};

    pub struct IndividualGroup {
        group: Vec<Individual>,
        inf_data: InfectionData,
        group_size: u32,

        field_max_x: u32,
        field_max_y: u32,
    }

    impl IndividualGroup {

        pub fn new (

        ) -> IndividualGroup {
            IndividualGroup {

            }
        }

        pub fn get_size(&self) -> u32 {
            self.group_size
        }

        pub fn get_positions(&self) -> Vec<(u32, u32)> {
            self.group.into_iter().map(
                |individual| {
                    individual.get_position()
                }
            ).collect()
        }

        pub fn make_turns(&self, turns_num: u32) {
            for _ in 0..turns_num {
                self.group.into_iter().map(
                    |individual| individual.make_turn()
                ).collect();
            }
        }

    }
}