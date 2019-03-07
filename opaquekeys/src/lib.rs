use std::error::Error;
use std::borrow::Cow;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyType {
    New,
    Old,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OpaqueKeyError;

impl std::fmt::Display for OpaqueKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "OpaqueKeyError")
    }
}

impl Error for OpaqueKeyError {
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CourseKey<'a> {
    key: Cow<'a, str>,
    keytype: KeyType,
}

impl<'a> serde::Serialize for CourseKey<'a> {
     fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
         s.serialize_str(&self.key)
     }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UsageKey<'a> {
    course_key: CourseKey<'a>,
    key: Cow<'a, str>,
}

impl<'a> serde::Serialize for UsageKey<'a> {
     fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
         s.serialize_str(&self.key)
     }
}


pub struct PartialUsageKey<'a> {
    key: Cow<'a, str>,
    keytype: KeyType,
}

impl<'a> serde::Serialize for PartialUsageKey<'a> {
     fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
         s.serialize_str(&self.key)
     }
}

impl<'a> CourseKey<'a> {
    pub fn new(org: &str, course: &str, run: &str, keytype: KeyType) -> CourseKey<'a> {
        match keytype {
            keytype @ KeyType::Old => {
                CourseKey {
                    key: Cow::from(format!("{}/{}/{}", org, course, run)),
                    keytype,
                }
            }
            keytype @ KeyType::New => {
                CourseKey {
                    key: Cow::from(format!("course-v1:{}+{}+{}", org, course, run)),
                    keytype,
                }
            }
        }
    }

    pub fn org(&self) -> &str {
        match self.keytype {
            KeyType::Old => self.key.split("/").next().unwrap(),
            KeyType::New => self.key["course-v1:".len()..].split("+").next().unwrap(),
        }
    }

    pub fn course(&self) -> &str {
        let iter = match self.keytype {
            KeyType::Old => self.key.split("/"),
            KeyType::New => self.key["course-v1:".len()..].split("+"),
        };
        iter.skip(1).next().unwrap()
    }

    pub fn run(&self) -> &str {
        let iter = match self.keytype {
            KeyType::Old => self.key.split("/"),
            KeyType::New => self.key["course-v1:".len()..].split("+"),
        };
        iter.skip(2).next().unwrap()
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn make_usage_key<'r, 'b>(&'r self, blocktype: &'b str, name: &'b str) -> UsageKey<'a> {
        UsageKey::from_parts(self.to_owned(), blocktype, name)
    }
}

impl<'a> std::str::FromStr for CourseKey<'a> {
    type Err = OpaqueKeyError;

    fn from_str(key: &str) -> Result<Self, Self::Err> {
        // Validate input!
        let keytype = if key.starts_with("course-v1:") {
            KeyType::New
        } else {
            KeyType::Old
        };
        Ok(CourseKey {
            key: Cow::from(key),
            keytype,
        })
    }
}

impl<'a> std::fmt::Display for CourseKey<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.key())
    }
}

impl<'a> UsageKey<'a> {
    pub fn new<K: Into<Cow<'a, str>>>(course_key: CourseKey<'a>, key: K) -> UsageKey<'a> {
        UsageKey {
            course_key,
            key: Cow::from(key),
        }
    }

    pub fn from_parts<'r, 'b>(course_key: &'r CourseKey<'a>, blocktype: &'b str, name: &'b str) -> UsageKey<'a> {
        UsageKey::new(
            course_key.clone(),
            match &course_key.keytype {
                &KeyType::Old => format!("i4x://{}/{}/{}/{}", course_key.org(), course_key.course(), blocktype, name),
                &KeyType::New => format!("block-v1:{}+{}+{}+type@{}+block@{}", course_key.org(), course_key.course(), course_key.run(), blocktype, name),
            }
        )
    }

    pub fn keytype(&self) -> KeyType {
        self.course_key.keytype
    }

    pub fn course_key(&self) -> &CourseKey {
        &self.course_key
    }

    pub fn org(&self) -> &str {
        self.course_key.org()
    }

    pub fn course(&self) -> &str {
        self.course_key.course()
    }

    pub fn run(&self) -> &str {
        self.course_key.run()
    }

    pub fn blocktype(&self) -> &str {
        match self.keytype() {
            KeyType::Old => self.key["i4x://".len()..].split("/").skip(2).next().unwrap(),
            KeyType::New => {
                let field = "type@";
                let blocktype_field = self.key.split("+").skip(3).next().unwrap();
                let (field_tag, value) = blocktype_field.split_at(field.len());
                debug_assert!(field_tag == field);
                value
            }
        }
    }

    pub fn block(&self) -> &str {
        match self.keytype() {
            KeyType::Old => self.key["i4x://".len()..].split("/").skip(3).next().unwrap(),
            KeyType::New => {
                let field = "block@";
                let blocktype_field = self.key.split("+").skip(4).next().unwrap();
                let (field_tag, value) = blocktype_field.split_at(field.len());
                debug_assert!(field_tag == field);
                value
            }
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    /// Partial Equality.  If a fully resolved usage key is a member of a course
    pub fn in_course(&self, checked: &CourseKey) -> bool {
        &self.course_key == checked
    }

}


impl<'a> std::fmt::Display for UsageKey<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.key())
    }
}


impl<'a> PartialUsageKey<'a> {
    pub fn try_promote(&self) -> Option<UsageKey<'a>> {
        match self.keytype {
            KeyType::Old => None,
            KeyType::New => {
                let mut iter = self.key["block-v1:".len()..].split("+");
                let org = iter.next().unwrap();
                let course = iter.next().unwrap();
                let run = iter.next().unwrap();
                let course_key = CourseKey::new(org, course, run, KeyType::New);
                Some(UsageKey::new(course_key, self.key.clone()))
            }
        }
    }

    pub fn map_into_course(&self, course_key: CourseKey<'a>) -> UsageKey<'a> {
        UsageKey::new(course_key, Cow::from(self.key))
    }

    pub fn org(&self) -> &str {
        match self.keytype {
            KeyType::Old => self.key["i4x://".len()..].split('/').next().unwrap(),
            KeyType::New => self.key["block-v1:".len()..].split('+').next().unwrap(),
        }
    }

    pub fn course(&self) -> &str {
        match self.keytype {
            KeyType::Old => self.key["i4x://".len()..].split('/').skip(1).next().unwrap(),
            KeyType::New => self.key["block-v1:".len()..].split('+').skip(1).next().unwrap(),
        }
    }

    pub fn run(&self) -> Option<&str> {
        match self.keytype {
            KeyType::Old => None,
            KeyType::New => Some(self.key["block-v1:".len()..].split('+').skip(2).next().unwrap()),
        }
    }

    pub fn blocktype(&self) -> &str {
        match self.keytype {
            KeyType::Old => self.key["i4x://".len()..].split("/").skip(2).next().unwrap(),
            KeyType::New => {
                let field = "type@";
                let blocktype_field = self.key.split("+").skip(3).next().unwrap();
                let (field_tag, value) = blocktype_field.split_at(field.len());
                debug_assert!(field_tag == field);
                value
            }
        }
    }

    pub fn block(&self) -> &str {
        match self.keytype {
            KeyType::Old => self.key["i4x://".len()..].split("/").skip(3).next().unwrap(),
            KeyType::New => {
                let field = "block@";
                let blocktype_field = self.key.split("+").skip(4).next().unwrap();
                let (field_tag, value) = blocktype_field.split_at(field.len());
                debug_assert!(field_tag == field);
                value
            }
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }
}


impl<'a> std::str::FromStr for PartialUsageKey<'a> {
    type Err = OpaqueKeyError;

    fn from_str(key: &str) -> Result<Self, Self::Err> {
        if key.starts_with("block-v1:") {
            let chunks: Vec<&str> = key[9..].split("+").collect();
            if chunks.len() == 5 {
                let _org = chunks.get(0).ok_or(OpaqueKeyError)?;
                let _course = chunks.get(1).ok_or(OpaqueKeyError)?;
                let _run = chunks.get(2).ok_or(OpaqueKeyError)?;
                let blocktype = chunks.get(3).ok_or(OpaqueKeyError)?;
                let name = chunks.get(4).ok_or(OpaqueKeyError)?;
                if !blocktype.starts_with("type@") || !name.starts_with("block@") {
                    Err(OpaqueKeyError)
                } else {
                    Ok(PartialUsageKey { key: Cow::from(key), keytype: KeyType::New })
                }
            } else {
                Err(OpaqueKeyError)
            }
        } else if key.starts_with("i4x://") {
            let chunks: Vec<&str> = key[6..].split("/").collect();
            if chunks.len() == 4 {
                Ok(PartialUsageKey { key: Cow::from(key), keytype: KeyType::Old })
            } else {
                Err(OpaqueKeyError)
            }
        } else {
            Err(OpaqueKeyError)
        }
    }
}


impl<'a> std::fmt::Display for PartialUsageKey<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.key())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_course_key() {
        assert_eq!(
            "course-v1:edX+DemoX+Demo2018".parse::<CourseKey>().unwrap(),
            CourseKey::new("edX", "DemoX", "Demo2018", KeyType::New),
        );
        assert_eq!(
            "edX/DemoX/Demo2018".parse::<CourseKey>().unwrap(),
            CourseKey::new("edX", "DemoX", "Demo2018", KeyType::Old),
        );
    }

    #[test]
    fn course_key_roundtrip() {
        for keystr in vec!["course-v1:edX+DemoX+Demo2018", "edX/DemoX/Demo2018"] {
            assert_eq!(
                format!("{}", keystr.parse::<CourseKey>().unwrap()),
                keystr,
            );
        }
    }

    #[test]
    fn basic_usage_key() {
        assert_eq!(
            "block-v1:edX+DemoX+Demo2018+type@html+block@introduction".parse::<PartialUsageKey>()
                .unwrap()
                .try_promote()
                .unwrap(),
            UsageKey::from_parts("course-v1:edX+DemoX+Demo2018".parse().unwrap(), "html", "introduction")
        )
    }


}
