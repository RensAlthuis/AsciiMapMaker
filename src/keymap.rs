use std::collections::BTreeMap;

type Key = char;

type Callback<F> = fn(context : &mut F);

pub struct KeyMap<F>{
    map : BTreeMap<Key, Callback<F>>
}

impl<F> KeyMap<F>{
    pub fn new() -> Self {
        let map = BTreeMap::new();
        KeyMap{
            map
        }
    }

    pub fn register(&mut self, key : Key, func : Callback<F>){
        self.map.insert(key, func);
    }

    pub fn call(&mut self, key : Key, context : &mut F) {
        if let Some(callback) = self.map.get(&key) {
            callback(context);
        };
    }
}