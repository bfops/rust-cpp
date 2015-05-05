pub type Name = String;
pub type TemplateParam = String;
pub type Param = String;
pub type ReturnType = String;

pub enum FunctionSignature {
  Simple(Name, ReturnType, Vec<TemplateParam>, Vec<Param>),
}
