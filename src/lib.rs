use std::vec::Vec;

use ndarray::Array2;
use strum_macros::EnumIter;
use strum_macros::Display;

#[derive(Debug, Display, Clone, EnumIter, PartialEq)]
pub enum ModelParameter
{
    Int(i32),
    Double(f64),
    String(String),
    ArrayInt(Vec<i32>),
    ArrayDouble(Vec<f64>),
    ArrayString(Vec<String>),
    MatrixInt(Array2<i32>),
    MatrixDouble(Array2<f64>)
}

pub trait Model
{
    fn run(&self, params: Vec<ModelParameter>);

    fn export(&self);

    fn get_score(&self) -> f64;
}

pub struct ParameterSweeper<T>
where T: Model
{
    pub model_under_test: T
}

impl <T: Model> ParameterSweeper<T>
{
   pub fn execute_model(&self, params: Vec<ModelParameter>) 
   {
       self.model_under_test.run(params);
   }

   pub fn export_model(&self)
   {
        self.model_under_test.export();
   }

   pub fn get_score(&self) -> f64
   {
        return self.model_under_test.get_score();
   }
}
