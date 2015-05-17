pub type Name = String;
pub type Type = String;
pub type TemplateParam = String;
pub type Param = String;
pub type ReturnType = String;

pub enum Binding {
  FreeFunction(Name, ReturnType, Vec<TemplateParam>, Vec<Param>),
  Struct(Name, Vec<TemplateParam>, Vec<Name>),
}
