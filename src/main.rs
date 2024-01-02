use parameter_sweeper::ParameterSweeper;
use parameter_sweeper::Model;
use parameter_sweeper::ModelParameter;

use ndarray::array;

pub struct MyModel
{
}

impl Model for MyModel
{
    fn run(&self, params: Vec<ModelParameter>)
    {
        println!("Running!\n");

        MyModel::match_parameter(params);
    }

    fn export(&self)
    {
        println!("Exporting!\n")
    }

    fn get_score(&self) -> f64
    {
        println!("Score: {}\n", 45.);
        45. 
    }
}

impl MyModel
{
    pub fn match_parameters(params: Vec<ModelParameter>) 
    {
        for (index, param) in params.iter().enumerate()
        {
            match param
            {
                ModelParameter::Int(value) if (index == 0) => println!("We found the int at index 0 so we know what it is {}", value),
                ModelParameter::Int(value) => println!("Parameter at index {} is of type {} and of value {}\n", index, param, value),
                ModelParameter::Double(value) => println!("Parameter at index {} is of type {} and of value {}\n", index, param, value),
                ModelParameter::String(value) => println!("Parameter at index {} is of type {} and of value {}\n", index, param, value),
                ModelParameter::ArrayInt(value) => println!("Parameter at index {} is of type {} and of value {:?}\n", index, param, value),
                ModelParameter::ArrayDouble(value) => println!("Parameter at index {} is of type {} and of value {:?}\n", index, param, value),
                ModelParameter::ArrayString(value) => println!("Parameter at index {} is of type {} and of value {:?}\n", index, param, value),
                ModelParameter::MatrixInt(value) => println!("Parameter at index {} is of type {} and of value {:?}\n", index, param, value),
                ModelParameter::MatrixDouble(value) => println!("Parameter at index {} is of type {} and of value {:?}\n", index, param, value)
            }
         
        }
    }
}

fn main() 
{
    println!("Hello, world!");
    
    let ps: ParameterSweeper<MyModel> = { ParameterSweeper { model_under_test: MyModel{} } };

    let params: Vec<ModelParameter> = vec![ModelParameter::Int(1), 
                                           ModelParameter::Double(3.14), 
                                           ModelParameter::String("blah".to_string()), 
                                           ModelParameter::ArrayDouble(vec![ 4.5, 6.5]), 
                                           ModelParameter::MatrixDouble(array![[1.2, 2.2], [2.3, 4.3]])];

    ps.execute_model(params);
    ps.export_model();
    ps.get_score();
}
