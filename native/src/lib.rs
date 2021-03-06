#[macro_use]
extern crate neon;
extern crate fuzzy_phrase;
extern crate neon_serde;

use neon::mem::Handle;
use neon::vm::{This, Lock, FunctionCall, JsResult};
use neon::js::{JsFunction, Object, JsString, JsNumber, Value, JsUndefined, JsArray, JsBoolean, JsInteger, JsObject};
use neon::js::class::{JsClass, Class};
use neon::js::error::{Kind, JsError};

use fuzzy_phrase::glue::{FuzzyPhraseSetBuilder, FuzzyPhraseSet};

trait CheckArgument {
    fn check_argument<V: Value>(&mut self, i: i32) -> JsResult<V>;
}

impl<'a, T: This> CheckArgument for FunctionCall<'a, T> {
    fn check_argument<V: Value>(&mut self, i: i32) -> JsResult<V> {
        self.arguments.require(self.scope, i)?.check::<V>()
    }
}

declare_types! {
    pub class JsFuzzyPhraseSetBuilder as JsFuzzyPhraseSetBuilder for Option<FuzzyPhraseSetBuilder> {
        init(mut call) {
            let filename = call
                .check_argument::<JsString>(0)
                ?.value();
            match FuzzyPhraseSetBuilder::new(filename){
                Ok(response) => {
                    Ok(Some(response))
                },
                Err(e) => {
                    println!("{:?}", e);
                    JsError::throw(Kind::TypeError, e.description())
                }
            }
        }

        method insert(call) {
            let phrase_array = call.arguments.require(call.scope, 0)?.check::<JsArray>()?;

            let mut v: Vec<String> = Vec::new();

            for i in 0..phrase_array.len() {
                let string = phrase_array.get(call.scope, i)
                ?.check::<JsString>()
                ?.value();

                v.push(string);
            }

            let mut this: Handle<JsFuzzyPhraseSetBuilder> = call.arguments.this(call.scope);

            this.grab(|fuzzyphrasesetbuilder| {
                match fuzzyphrasesetbuilder {
                    Some(builder) => {
                        match builder.insert(v.as_slice()) {
                            Ok(()) => {
                                Ok(JsUndefined::new().upcast())
                            },
                            Err(e) => {
                                println!("{:?}", e);
                                JsError::throw(Kind::TypeError, e.description())
                            }
                        }
                    },
                    None => {
                        JsError::throw(Kind::TypeError, "unable to insert()")
                    }
                }
            })
        }

        method finish(call) {
            let scope = call.scope;
            let mut this: Handle<JsFuzzyPhraseSetBuilder> = call.arguments.this(scope);

            this.grab(|fuzzyphrasesetbuilder| {
                match fuzzyphrasesetbuilder.take() {
                    Some(builder) => {
                        match builder.finish() {
                            Ok(_finish) => {
                                Ok(JsUndefined::new().upcast())
                            },
                            Err(e) => {
                                println!("{:?}", e);
                                JsError::throw(Kind::TypeError, e.description())
                            }
                        }
                    },
                    None => {
                        JsError::throw(Kind::TypeError, "unable to finish()")
                    }
                }
            })
        }
    }

    pub class JsFuzzyPhraseSet as JsFuzzyPhraseSet for FuzzyPhraseSet {
        init(mut call) {
            let filepath = call
                .check_argument::<JsString>(0)
                ?.value();
            match FuzzyPhraseSet::from_path(filepath) {
                Ok(set) => {
                    Ok(set)
                },
                Err(e) => {
                    println!("{:?}", e);
                    JsError::throw(Kind::TypeError, e.description())
                }
            }
        }

        method contains(call) {
            let phrase_array = call.arguments.require(call.scope, 0)?.check::<JsArray>()?;

            let mut v: Vec<String> = Vec::new();

            for i in 0..phrase_array.len() {
                let string = phrase_array.get(call.scope, i)
                ?.check::<JsString>()
                ?.value();

                v.push(string);
            }

            let mut this: Handle<JsFuzzyPhraseSet> = call.arguments.this(call.scope);

            let result = this.grab(|set| {
                match set.contains(v.as_slice()) {
                    Ok(response) => {
                        Ok(response)
                    },
                    Err(e) => {
                        println!("{:?}", e);
                        JsError::throw(Kind::TypeError, e.description())
                    }
                }
            });

            Ok(JsBoolean::new(
                call.scope,
                result?
            ).upcast())
        }

        method containsPrefix(call) {
            let phrase_array = call.arguments.require(call.scope, 0)?.check::<JsArray>()?;

            let mut v: Vec<String> = Vec::new();

            for i in 0..phrase_array.len() {
                let string = phrase_array.get(call.scope, i)
                ?.check::<JsString>()
                ?.value();

                v.push(string);
            }

            let mut this: Handle<JsFuzzyPhraseSet> = call.arguments.this(call.scope);

            let result = this.grab(|set| {
                match set.contains_prefix(v.as_slice()) {
                    Ok(response) => {
                        Ok(response)
                    },
                    Err(e) => {
                        println!("{:?}", e);
                        JsError::throw(Kind::TypeError, e.description())
                    }
                }
            });

            Ok(JsBoolean::new(
                call.scope,
                result?
            ).upcast())
        }

        method fuzzyMatch(call) {
            let phrase_array = call.arguments.require(call.scope, 0)?.check::<JsArray>()?;
            let max_word_dist: u8 = call.arguments.require(call.scope, 1)?.check::<JsInteger>()
                ?.value() as u8;
            let max_phrase_dist: u8 = call.arguments.require(call.scope, 2)?.check::<JsInteger>()
                ?.value() as u8;

            let mut v: Vec<String> = Vec::new();

            for i in 0..phrase_array.len() {
                let string = phrase_array.get(call.scope, i)
                ?.check::<JsString>()
                ?.value();

                v.push(string);
            }

            let mut this: Handle<JsFuzzyPhraseSet> = call.arguments.this(call.scope);

            let result = this.grab(|set| {
                set.fuzzy_match(v.as_slice(), max_word_dist, max_phrase_dist)
            });

            match result {
                Ok(vec) => {
                    let array = JsArray::new(
                        call.scope,
                        vec.len() as u32
                    );
                    for (i, item) in vec.iter().enumerate() {
                        let object = JsObject::new(
                            call.scope
                        );
                        let phrase = JsArray::new(
                            call.scope,
                            item.phrase.len() as u32
                        );
                        for (i, word) in item.phrase.iter().enumerate() {
                            let string = JsString::new_or_throw(
                                call.scope,
                                word
                            )?;
                            phrase.set(i as u32, string)?;
                        }
                        object.set("phrase", phrase)?;

                        let number = JsNumber::new(
                            call.scope,
                            item.edit_distance as f64
                        );
                        object.set("edit_distance", number)?;

                        array.set(i as u32, object)?;
                    }

                    Ok(array.upcast())
                },
                Err(e) => {
                    println!("{:?}", e);
                    JsError::throw(Kind::TypeError, e.description())
                }
            }
        }

        method fuzzyMatchPrefix(call) {
            let phrase_array = call.arguments.require(call.scope, 0)?.check::<JsArray>()?;
            let max_word_dist: u8 = call.arguments.require(call.scope, 1)?.check::<JsInteger>()
                ?.value() as u8;
            let max_phrase_dist: u8 = call.arguments.require(call.scope, 2)?.check::<JsInteger>()
                ?.value() as u8;

            let mut v: Vec<String> = Vec::new();

            for i in 0..phrase_array.len() {
                let string = phrase_array.get(call.scope, i)
                ?.check::<JsString>()
                ?.value();

                v.push(string);
            }

            let mut this: Handle<JsFuzzyPhraseSet> = call.arguments.this(call.scope);

            let result = this.grab(|set| {
                set.fuzzy_match_prefix(v.as_slice(), max_word_dist, max_phrase_dist)
            });

            match result {
                Ok(vec) => {
                    let array = neon_serde::to_value(call.scope, &vec)?;

                    Ok(array.upcast())
                },
                Err(e) => {
                    println!("{:?}", e);
                    JsError::throw(Kind::TypeError, e.description())
                }
            }
        }

        method fuzzyMatchWindows(call) {
            let phrase_array = call.arguments.require(call.scope, 0)?.check::<JsArray>()?;
            let max_word_dist: u8 = call.arguments.require(call.scope, 1)?.check::<JsInteger>()
                ?.value() as u8;
            let max_phrase_dist: u8 = call.arguments.require(call.scope, 2)?.check::<JsInteger>()
                ?.value() as u8;
            let ends_in_prefix: bool = call.arguments.require(call.scope, 3)?.check::<JsBoolean>()
                ?.value();

            let mut v: Vec<String> = Vec::new();

            for i in 0..phrase_array.len() {
                let string = phrase_array.get(call.scope, i)
                ?.check::<JsString>()
                ?.value();

                v.push(string);
            }

            let mut this: Handle<JsFuzzyPhraseSet> = call.arguments.this(call.scope);

            let result = this.grab(|set| {
                set.fuzzy_match_windows(v.as_slice(), max_word_dist, max_phrase_dist, ends_in_prefix)
            });

            match result {
                Ok(vec) => {
                    let array = neon_serde::to_value(call.scope, &vec)?;

                    Ok(array.upcast())
                },
                Err(e) => {
                    println!("{:?}", e);
                    JsError::throw(Kind::TypeError, e.description())
                }
            }
        }

        method fuzzyMatchMulti(call) {
            let arg0 = call.arguments.require(call.scope, 0)?;
            let multi_array: Vec<(Vec<String>, bool)> = neon_serde::from_value(
                call.scope,
                arg0
            )?;

            let max_word_dist: u8 = call.arguments.require(call.scope, 1)?.check::<JsInteger>()
                ?.value() as u8;
            let max_phrase_dist: u8 = call.arguments.require(call.scope, 2)?.check::<JsInteger>()
                ?.value() as u8;

            let mut this: Handle<JsFuzzyPhraseSet> = call.arguments.this(call.scope);

            let result = this.grab(|set| {
                set.fuzzy_match_multi(multi_array.as_slice(), max_word_dist, max_phrase_dist)
            });

            match result {
                Ok(vec) => {
                    let array = neon_serde::to_value(call.scope, &vec)?;

                    Ok(array.upcast())
                },
                Err(e) => {
                    println!("{:?}", e);
                    JsError::throw(Kind::TypeError, e.description())
                }
            }
        }
    }
}

register_module!(m, {

    let class: Handle<JsClass<JsFuzzyPhraseSetBuilder>> = try!(JsFuzzyPhraseSetBuilder::class(m.scope));
    let constructor: Handle<JsFunction<JsFuzzyPhraseSetBuilder>> = try!(class.constructor(m.scope));
    try!(m.exports.set("FuzzyPhraseSetBuilder", constructor));

    let class: Handle<JsClass<JsFuzzyPhraseSet>> = try!(JsFuzzyPhraseSet::class(m.scope));
    let constructor: Handle<JsFunction<JsFuzzyPhraseSet>> = try!(class.constructor(m.scope));
    try!(m.exports.set("FuzzyPhraseSet", constructor));

    Ok(())
});
