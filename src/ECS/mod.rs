#![allow(dead_code)]

pub mod comp;
pub mod storage;
pub mod system;
pub mod world;
pub mod resource;
pub mod dispatcher;
pub mod entity;
pub mod events;
pub mod commands;
pub mod fetch;

pub mod prelude;

#[cfg(tests)]
pub mod tests{
    use super::*;

    use event::*;
    use time::Duration;

    use prelude::*;

    pub fn main(){
        let mut world = gmWorld::new();

        world.registerComp::<gmComp_Health>();
        world.registerComp::<gmComp_Pos>();
        world.registerComp::<gmComp_Vel>();
        
        world.registerRes::<gmRes_deltaT>();
        world.registerRes::<gmRes_PInput>();
        
        world.createGmObj()
            .addComp::<gmComp_Health>(gmComp_Health{val: 100})
            .addComp::<gmComp_Pos>(gmComp_Pos{x: 0, y: 0})
            .finish();

        let mut dispatcher = gmDispatcher::new()
            .withSys::<gmSys_input>()
            .withSys::<gmSys_HP>()
            .withSys::<gmSys_movement>();

        dispatcher.dispatch(&mut world);

        let _ = world.deleteGmObj(0);

        println!("Success!");
    }

    pub struct gmComp_Health{
        pub val: usize
    }
    impl gmComp for gmComp_Health{
        type COMP_STORAGE = denseVecStorage<Self>;
        fn COMP_ID() -> &'static str {
            "gmComp_Health"
        }
    }

    pub struct gmComp_Pos{
        pub x: usize,
        pub y: usize
    }
    impl gmComp for gmComp_Pos{
        type COMP_STORAGE = denseVecStorage<Self>;
        fn COMP_ID() -> &'static str {
            "gmComp_Pos"
        }
    }

    pub struct gmComp_Vel{
        pub x: usize,
        pub y: usize
    }
    impl gmComp for gmComp_Vel{
        type COMP_STORAGE = denseVecStorage<Self>;
        fn COMP_ID() -> &'static str {
            "gmComp_Vel"
        }
    }

    pub struct gmSys_HP{}
    impl<'a> gmSystem<'a> for gmSys_HP{
        type sysData = gmSysData_HP<'a>;

        const sysDepends: &'static [&'static str] = &[];

        fn new() -> Self {
            Self{}
        }

        fn SYS_ID() -> &'static str {
            "gmSys_HP"
        }

        fn execute(&mut self, mut IN_data: Self::sysData) {
            for COMP_HP in IN_data.comp_HP.inner.iter_mut(){
                if COMP_HP.val.val > 0{
                    COMP_HP.val.val -= 1
                }
            }
        }
    }
    pub struct gmSysData_HP<'a>{
        pub comp_HP: WriteStorage<'a, gmComp_Health>
    }
    impl<'a> gmSystemData<'a> for gmSysData_HP<'a>{
        fn fetch(IN_world: &'a mut gmWorld) -> Self {
            Self{
                comp_HP: IN_world.fetchMut::<gmComp_Health>()
            }
        }
    }

    pub struct gmSys_input{}
    impl<'a> gmSystem<'a> for gmSys_input{
        type sysData = gmSysData_Input<'a>;

        const sysDepends: &'static [&'static str] = &[];

        fn new() -> Self {
            Self{}
        }

        fn SYS_ID() -> &'static str {
            "gmSys_input"
        }

        fn execute(&mut self, mut IN_data: Self::sysData) {
            if !poll(Duration::from_secs(0)).unwrap(){
                IN_data.res_Input.res = KeyEvent::new(KeyCode::Null, KeyModifiers::NONE);
                return
            }

            if let Event::Key(KEY) = read().unwrap(){
                IN_data.res_Input.res = KEY;
                return
            }
        }
    }
    pub struct gmSysData_Input<'a>{
        pub res_Input: FetchMut<'a, gmRes_PInput>
    }
    impl<'a> gmSystemData<'a> for gmSysData_Input<'a>{
        fn fetch(IN_world: &'a mut gmWorld) -> Self {
            Self{
                res_Input: IN_world.fetchResMut()
            }
        }
    }

    pub struct gmSys_movement{}
    impl<'a> gmSystem<'a> for gmSys_movement{
        type sysData = gmSysData_Movement<'a>;

        const sysDepends: &'static [&'static str] = &[];
    
        fn new() -> Self {
            Self{}
        }
    
        fn SYS_ID() -> &'static str {
            "gmSys_movement"
        }
    
        fn execute(&mut self, _IN_data: Self::sysData) {
            
        }
    }
    pub struct gmSysData_Movement<'a>{
        pub comp_pos: WriteStorage<'a, gmComp_Pos>,
        pub comp_vel: WriteStorage<'a, gmComp_Vel>,
    }
    impl<'a> gmSystemData<'a> for gmSysData_Movement<'a>{
        fn fetch(IN_world: &'a mut gmWorld) -> Self {
            Self{
                comp_pos: IN_world.fetchMut(),
                comp_vel: IN_world.fetchMut(),
            }
        }
    }

    pub struct gmRes_deltaT{
        res: Duration
    }
    impl gmRes for gmRes_deltaT{
        fn new() -> Self {
            Self{
                res: Duration::from_secs(0)
            }
        }
        fn RES_ID() -> &'static str {
            "gmRes_deltaT"
        }
    }
    impl Deref for gmRes_deltaT{
        type Target = Duration;
    
        fn deref(&self) -> &Self::Target {
            &self.res
        }
    }
    impl DerefMut for gmRes_deltaT{
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.res
        }
    }

    pub struct gmRes_PInput{
        res: KeyEvent
    }
    impl gmRes for gmRes_PInput{
        fn new() -> Self {
            Self{
                res: KeyEvent{
                    code: KeyCode::Null,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Release,
                    state: KeyEventState::NONE,
                }
            }
        }
        fn RES_ID() -> &'static str {
            "gmResPInput"
        }
    }
    impl Deref for gmRes_PInput {
        type Target = KeyEvent;
    
        fn deref(&self) -> &Self::Target {
            &self.res
        }
    }
    impl DerefMut for gmRes_PInput{
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.res
        }
    }
}