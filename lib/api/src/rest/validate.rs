use std::borrow::Cow;

use common::validation::validate_multi_vector;
use validator::{Validate, ValidationError, ValidationErrors};

use super::schema::{BatchVectorStruct, Vector, VectorStruct};
use super::{
    ContextInput, Fusion, OrderByInterface, Query, QueryInterface, RecommendInput, Sample,
    VectorInput,
};
use crate::rest::NamedVectorStruct;

impl Validate for VectorStruct {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            VectorStruct::Single(_) => Ok(()),
            VectorStruct::MultiDense(v) => validate_multi_vector(v),
            VectorStruct::Named(v) => common::validation::validate_iter(v.values()),
            VectorStruct::Document(_) => {
                let mut errors = ValidationErrors::default();
                let mut err = ValidationError::new("not_supported_inference");
                err.add_param(
                    Cow::from("message"),
                    &"Document inference is not implemented, please use vectors instead",
                );
                errors.add("text", err);
                Err(errors)
            }
        }
    }
}

impl Validate for BatchVectorStruct {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            BatchVectorStruct::Single(_) => Ok(()),
            BatchVectorStruct::MultiDense(vectors) => {
                for vector in vectors {
                    common::validation::validate_multi_vector(vector)?;
                }
                Ok(())
            }
            BatchVectorStruct::Named(v) => {
                common::validation::validate_iter(v.values().flat_map(|batch| batch.iter()))
            }
            BatchVectorStruct::Document(_) => {
                let mut errors = ValidationErrors::default();
                let mut err = ValidationError::new("not_supported_inference");
                err.add_param(
                    Cow::from("message"),
                    &"Document inference is not implemented, please use vectors instead",
                );
                errors.add("text", err);
                Err(errors)
            }
        }
    }
}

impl Validate for Vector {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            Vector::Dense(_) => Ok(()),
            Vector::Sparse(v) => v.validate(),
            Vector::MultiDense(m) => common::validation::validate_multi_vector(m),
            Vector::Document(_) => {
                let mut errors = ValidationErrors::default();
                let mut err = ValidationError::new("not_supported_inference");
                err.add_param(
                    Cow::from("message"),
                    &"Document inference is not implemented, please use vectors instead",
                );
                errors.add("text", err);
                Err(errors)
            }
        }
    }
}

impl Validate for NamedVectorStruct {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            NamedVectorStruct::Default(_) => Ok(()),
            NamedVectorStruct::Dense(_) => Ok(()),
            NamedVectorStruct::Sparse(v) => v.validate(),
        }
    }
}

impl Validate for QueryInterface {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            QueryInterface::Nearest(vector) => vector.validate(),
            QueryInterface::Query(query) => query.validate(),
        }
    }
}

impl Validate for Query {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            Query::Nearest(vector) => vector.nearest.validate(),
            Query::Recommend(recommend) => recommend.recommend.validate(),
            Query::Discover(discover) => discover.discover.validate(),
            Query::Context(context) => context.context.validate(),
            Query::Fusion(fusion) => fusion.fusion.validate(),
            Query::OrderBy(order_by) => order_by.order_by.validate(),
            Query::Sample(sample) => sample.sample.validate(),
        }
    }
}

impl Validate for VectorInput {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            VectorInput::Id(_id) => Ok(()),
            VectorInput::DenseVector(_dense) => Ok(()),
            VectorInput::SparseVector(sparse) => sparse.validate(),
            VectorInput::MultiDenseVector(multi) => validate_multi_vector(multi),
            VectorInput::Document(_) => {
                let mut errors = ValidationErrors::default();
                let mut err = ValidationError::new("not_supported_inference");
                err.add_param(
                    Cow::from("message"),
                    &"Document inference is not implemented, please use vectors instead",
                );
                errors.add("text", err);
                Err(errors)
            }
        }
    }
}

impl Validate for RecommendInput {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        let no_positives = self.positive.as_ref().map(|p| p.is_empty()).unwrap_or(true);
        let no_negatives = self.negative.as_ref().map(|n| n.is_empty()).unwrap_or(true);

        if no_positives && no_negatives {
            let mut errors = validator::ValidationErrors::new();
            errors.add(
                "positives, negatives",
                ValidationError::new(
                    "At least one positive or negative vector/id must be provided",
                ),
            );
            return Err(errors);
        }

        for item in self.iter() {
            item.validate()?;
        }

        Ok(())
    }
}

impl Validate for ContextInput {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        for item in self.0.iter().flatten().flat_map(|item| item.iter()) {
            item.validate()?;
        }

        Ok(())
    }
}

impl Validate for Fusion {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            Fusion::Rrf | Fusion::Dbsf => Ok(()),
        }
    }
}

impl Validate for OrderByInterface {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            OrderByInterface::Key(_key) => Ok(()), // validated during parsing
            OrderByInterface::Struct(order_by) => order_by.validate(),
        }
    }
}

impl Validate for Sample {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            Sample::Random => Ok(()),
        }
    }
}
