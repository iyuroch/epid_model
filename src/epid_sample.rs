// TODO: consider being immune to desease
// TODO: bacteriocarrier with some chance to become
// TODO: check desease status
// TODO: multiple infections
// TODO: speed of individual setter (we can change it for decrease)
// TODO: consider mutation for infection dat
// TODO: individalgroup perfect forwarding initializer
// TODO: consider box for vector of individuals
// TODO: extend individual group intitalization
// TODO: check spread_desease - we can do it faster
// TODO: get_status - not clears
// TODO: random??
// TODO: restriction to contagious index
// TODO: running away from contagious
// TODO: concurrency
// TODO: cargo test - rewrite
// TODO: status -> can be useful
// TODO: metadata per individual

// TODO: birthray/deathrate
// TODO: immune system fighting off
// TODO: vaccination

// TODO: dynamic of desease

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
    #[derive(Serialize)]
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
        pub fn get_des_day(&self) -> Option<u32> {
            self.desease_day
        }

        #[allow(dead_code)]
        pub fn get_status(&self) -> bool {
            self.contagious
        }

        fn walk(&mut self, diff: (i32, i32)) {
            // should be able to move individual
            // on the field
            let (x_diff, y_diff) = diff;
            let new_x_pos = self.x as i32 + x_diff;
            let new_y_pos = self.y as i32 + y_diff;

            if new_x_pos >= 0 && new_x_pos < self.max_x {
                self.x = new_x_pos as u32;
            }

            if new_y_pos >= 0 && new_y_pos < self.max_y {
                self.y = new_y_pos as u32;
            }
        }

        fn generate_move(&self) -> (i32, i32) {
            let mut rng = rand::thread_rng();
            let range = (-1 * self.speed as i32, 1 * self.speed);
            let x_diff = rng.gen_range(range.0 as i32, range.1 as i32 + 1);
            let y_diff = rng.gen_range(range.0 as i32, range.1 as i32 + 1);
            return (x_diff, y_diff)
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
    extern crate rayon;
    use rayon::prelude::*;

    extern crate rand;
    use rand::Rng;

    use super::individual::{Individual, InfectionData};

    pub struct IndividualGroup {
        group: Vec<Individual>,
        inf_data: InfectionData,
        group_size: u32,
        field_max_x: i32,
        field_max_y: i32,
    }

    #[derive(Serialize)]
    pub struct GroupMetadata {
        inf_data: InfectionData,
        field_max_x: i32,
        field_max_y: i32,
    }

    #[test]
    fn test_constructor() {
        let inf_num = 3;
        let inf_data = InfectionData::new(
            2, 0.8, 1, 2,
        );

        let group = IndividualGroup::new(
            100, 100, 2,
            10, inf_num , inf_data,
        );

        let ind_data = group.get_individuals();
        let inf_num_count = ind_data.into_iter().filter(
            |data| {
                data.1 == true
            }
        ).count();
        assert_eq!(inf_num as usize, inf_num_count);
    }

    #[test]
    #[should_panic(expected = "inf_num \
             cannot be bigger than group_size")]
    fn test_fail_constructor() {
        let inf_data = InfectionData::new(
            2, 0.8, 1, 2,
        );

        IndividualGroup::new(
            100, 100, 2,
            3, 10,
            inf_data,
        );
    }

    #[test]
    fn test_spread_desease() {
        // if we have no other place to 
        // go - individuals in this case close to each other
        // and all get infected
        let inf_data = InfectionData::new(
            2, 1.0, 1, 2,
        );

        let mut group = IndividualGroup::new(
            3, 3, 2,
            9, 0, inf_data,
        );

        group.spread_desease();
        let ind_data = group.get_data();
        let inf_num_count = ind_data.into_iter().filter(
            |ind| {
                ind.get_des_day().is_some()
            }
        ).count();

        assert_eq!(9, inf_num_count);
    }

    impl IndividualGroup {

        #[allow(dead_code)]
        pub fn new (
            // inf_num - amount of infected individuals
            max_x: i32,
            max_y: i32,
            ind_speed: u32,
            group_size: u32,
            mut inf_num: u32,
            inf_data: InfectionData,
        ) -> IndividualGroup {
            assert!(inf_num <= group_size, "inf_num \
             cannot be bigger than group_size");

            let mut rng = rand::thread_rng();
            let mut group = Vec::with_capacity(group_size as usize);

            for _ in 0..group_size {
                
                let x_pos = rng.gen_range(0, max_x) as u32;
                let y_pos = rng.gen_range(0, max_y) as u32;

                let mut should_inf = None;

                if inf_num > 0 {
                    inf_num -= 1;
                    should_inf = Some(inf_data.incubation_period + 1);
                }
                
                group.push(Individual::new(
                    x_pos, y_pos, ind_speed,
                    should_inf, max_x, max_y,
                    inf_data,
                ));
            }
            IndividualGroup {
                group: group,
                inf_data: inf_data,
                group_size: group_size,
                field_max_x: max_x,
                field_max_y: max_y,
            }
        }

        pub fn get_group_metadata(&self) -> GroupMetadata {
            let metadata = GroupMetadata {
                inf_data: self.inf_data,
                field_max_x: self.field_max_x,
                field_max_y: self.field_max_y,
            };
            return metadata;
        }


        #[allow(dead_code)]
        pub fn get_size(&self) -> u32 {
            self.group_size
        }

        #[allow(dead_code)]
        pub fn get_individuals(&self) -> Vec<((u32, u32), bool, Option<u32>)> {
            self.group.iter().map(
                |individual| {
                    (individual.get_position(),
                     individual.get_status(),
                     individual.get_des_day(),
                    )
                }
            ).collect()
        }

        #[allow(dead_code)]
        pub fn par_get_individuals(&self) -> Vec<((u32, u32), bool, Option<u32>)> {
            self.group.par_iter().map(
                |individual| {
                    (individual.get_position(),
                     individual.get_status(),
                     individual.get_des_day(),
                    )
                }
            ).collect()
        }

        #[allow(dead_code)]
        pub fn get_data(&self) -> &Vec<Individual> {
            &self.group
        }

        fn spread_desease(&mut self) {
            let mut rng = rand::thread_rng();

            let mut inf_area = vec![
                                vec![0; self.field_max_x as usize]; 
                                self.field_max_y as usize
                            ];
            let infected: Vec<&Individual> = self.group.iter().filter(
                |individual| individual.get_status() == false
            ).collect();

            for ind in infected.into_iter() {
                let (x, y) = ind.get_position();

                let x_st_range = x as i32 - self.inf_data.contagious_range as i32;
                let x_end_range = x as i32 + self.inf_data.contagious_range as i32;

                let y_st_range = y as i32 - self.inf_data.contagious_range as i32;
                let y_end_range = y as i32 + self.inf_data.contagious_range as i32;

                for x in x_st_range..x_end_range {
                    for y in y_st_range..y_end_range {
                        if (x >= 0 && x < self.field_max_x) && 
                            (y >= 0 && y< self.field_max_y) {
                            inf_area[y as usize][x as usize] = 1;
                        }
                    }
                }
            }

            for ind in self.group.iter_mut() {
                if ind.get_des_day().is_none() {
                    let (x, y) = ind.get_position();
                    if inf_area[y as usize][x as usize] == 1 {
                        if rng.gen_range(0.0, 1.0) < self.inf_data.contagious_ind {
                            ind.make_infected();
                        }
                    }
                }
            }
        }

        #[allow(dead_code)]
        pub fn make_turns(&mut self, turns_num: u32) {
            for _ in 0..turns_num {
                self.spread_desease();
                self.group.iter_mut().for_each(
                    |individual| individual.make_turn()
                );
            }
        }
    }
}
