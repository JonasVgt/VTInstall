
#[derive(PartialEq, Debug)]
pub struct UseStatement{
    path: String,
}

impl UseStatement{
    pub fn parse(input: &str) -> Result<Self, String>{  
        Ok(UseStatement{path: String::from(input)})
    }
}

#[cfg(test)]
mod tests{
    
    use super::*;
    
}