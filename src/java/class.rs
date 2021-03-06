//! Data structure for classes.

use super::constructor::Constructor;
use super::field::Field;
use super::method::Method;
use super::modifier::Modifier;
use cons::Cons;
use element::Element;
use into_tokens::IntoTokens;
use java::Java;
use tokens::Tokens;

/// Model for Java Classs.
#[derive(Debug, Clone)]
pub struct Class<'el> {
    /// Class modifiers.
    pub modifiers: Vec<Modifier>,
    /// Declared methods.
    pub fields: Vec<Field<'el>>,
    /// Declared methods.
    pub constructors: Vec<Constructor<'el>>,
    /// Declared methods.
    pub methods: Vec<Method<'el>>,
    /// Extra body (at the end of the class).
    pub body: Tokens<'el, Java<'el>>,
    /// What this class extends.
    pub extends: Option<Java<'el>>,
    /// What this class implements.
    pub implements: Vec<Java<'el>>,
    /// Generic parameters.
    pub parameters: Tokens<'el, Java<'el>>,
    /// Annotations for the constructor.
    annotations: Tokens<'el, Java<'el>>,
    /// Name of class.
    name: Cons<'el>,
}

impl<'el> Class<'el> {
    /// Build a new empty interface.
    pub fn new<N>(name: N) -> Class<'el>
    where
        N: Into<Cons<'el>>,
    {
        Class {
            modifiers: vec![Modifier::Public],
            fields: vec![],
            methods: vec![],
            body: Tokens::new(),
            constructors: vec![],
            extends: None,
            implements: vec![],
            parameters: Tokens::new(),
            annotations: Tokens::new(),
            name: name.into(),
        }
    }

    /// Push an annotation.
    pub fn annotation<A>(&mut self, annotation: A)
    where
        A: IntoTokens<'el, Java<'el>>,
    {
        self.annotations.push(annotation.into_tokens());
    }

    /// Name of class.
    pub fn name(&self) -> Cons<'el> {
        self.name.clone()
    }
}

into_tokens_impl_from!(Class<'el>, Java<'el>);

impl<'el> IntoTokens<'el, Java<'el>> for Class<'el> {
    fn into_tokens(self) -> Tokens<'el, Java<'el>> {
        let mut sig = Tokens::new();

        sig.extend(self.modifiers.into_tokens());
        sig.append("class");

        sig.append({
            let mut t = Tokens::new();

            t.append(self.name.clone());

            if !self.parameters.is_empty() {
                t.append("<");
                t.append(self.parameters.join(", "));
                t.append(">");
            }

            t
        });

        if let Some(extends) = self.extends {
            sig.append("extends");
            sig.append(extends);
        }

        if !self.implements.is_empty() {
            let implements: Tokens<_> = self
                .implements
                .into_iter()
                .map::<Element<_>, _>(Into::into)
                .collect();

            sig.append("implements");
            sig.append(implements.join(", "));
        }

        let mut s = Tokens::new();

        if !self.annotations.is_empty() {
            s.push(self.annotations);
        }

        s.push(toks![sig.join_spacing(), " {"]);

        s.nested({
            let mut body = Tokens::new();

            if !self.fields.is_empty() {
                let mut fields = Tokens::new();

                for field in self.fields {
                    fields.push(toks![field, ";"]);
                }

                body.push(fields);
            }

            if !self.constructors.is_empty() {
                for constructor in self.constructors {
                    body.push((self.name.clone(), constructor));
                }
            }

            if !self.methods.is_empty() {
                for method in self.methods {
                    body.push(method);
                }
            }

            body.extend(self.body);
            body.join_line_spacing()
        });

        s.push("}");

        s
    }
}

#[cfg(test)]
mod tests {
    use super::Class;
    use java::{local, Java};
    use tokens::Tokens;

    #[test]
    fn test_vec() {
        let mut c = Class::new("Foo");
        c.parameters.append("T");
        c.implements = vec![local("Super").into()];

        let t: Tokens<Java> = c.into();

        let s = t.to_string();
        let out = s.as_ref().map(|s| s.as_str());
        assert_eq!(Ok("public class Foo<T> implements Super {\n}"), out);
    }
}
