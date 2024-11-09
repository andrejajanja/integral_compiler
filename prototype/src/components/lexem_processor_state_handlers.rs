use super::{
    lexem_processor_taylor::LexemProcessorTaylor,
    object_type_definitions::Func,
    polynomials::TsPoly
};

impl LexemProcessorTaylor{
    pub(crate) fn state_0_handler(&mut self){
        
    }

    pub(crate) fn state_1_handler(&mut self){

    }

    pub(crate) fn state_2_handler(&mut self){
        todo!("Evaluate {} on this function(s) {}", self.temp_const, self.current_lexem.op);
    }

    pub(crate) fn state_3_handler(&mut self){
        match self.current_lexem.op {
            Func::Add => {
                let mut temp_poly = TsPoly::new();
                temp_poly.coefs[1] = self.temp_const;
                temp_poly.coefs[1] = 1.0;
                temp_poly.max_pow = 1;
                self.gen_polys.push(temp_poly);
            },
            Func::Sub => {
                let mut temp_poly = TsPoly::new();
                temp_poly.coefs[1] = -self.temp_const;
                temp_poly.coefs[1] = 1.0;
                temp_poly.max_pow = 1;
                self.gen_polys.push(temp_poly);
            },
            Func::Mul => {
                let mut temp_poly = TsPoly::new();
                temp_poly.coefs[1] = self.temp_const;
                temp_poly.max_pow = 1;
                self.gen_polys.push(temp_poly);
            },
            Func::Div => {
                let mut temp_poly = TsPoly::new();
                temp_poly.coefs[1] = 1.0/self.temp_const; //FIXME Handle if temp_const is 0 here, to stop the entire thing
                temp_poly.max_pow = 1;
                self.gen_polys.push(temp_poly);
            },
            Func::Pow => todo!(), //TODO x^const
            _ => todo!()
        }
    }

    pub(crate) fn state_4_handler(&mut self){
        match self.current_lexem.op {
            Func::Add => {
                let mut temp_poly = TsPoly::new();
                temp_poly.coefs[1] = self.temp_const;
                temp_poly.coefs[1] = 1.0;
                temp_poly.max_pow = 1;
                self.gen_polys.push(temp_poly);
            },
            Func::Sub => {
                let mut temp_poly = TsPoly::new();
                temp_poly.coefs[1] = self.temp_const;
                temp_poly.coefs[1] = -1.0;
                temp_poly.max_pow = 1;
                self.gen_polys.push(temp_poly);
            },
            Func::Mul => {
                let mut temp_poly = TsPoly::new();
                temp_poly.coefs[1] = self.temp_const;
                temp_poly.max_pow = 1;
                self.gen_polys.push(temp_poly);
            },
            Func::Div => todo!(), //TOOD const/x
            Func::Pow => todo!(), //TODO const^x
            _ => todo!()
        }
    }


}