// Copyright 2022 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::lock_api::RwLock;

use common_ast::ast::ExplainKind;
use common_ast::{Backtrace, Dialect};
use common_ast::parser::parse_sql;
use common_ast::parser::token::{Token, Tokenizer};
use common_catalog::catalog::CatalogManager;
use common_catalog::table_context::TableContext;
use common_exception::ErrorCode;
use common_exception::Result;

use crate::optimizer::HeuristicOptimizer;
use crate::optimizer::SExpr;
use crate::optimizer::DEFAULT_REWRITE_RULES;
use crate::plans::CopyPlanV2;
use crate::plans::Plan;
use crate::{BindContext, Binder, Metadata, NameResolutionContext};
use crate::IndexType;
use crate::MetadataRef;

#[derive(Debug, Clone, Default)]
pub struct MaterialConfig {
    pub enable_material_cache: bool,
}

#[derive(Debug)]
pub struct MaterialContext {
    pub config: MaterialConfig,
    pub view: Arc<Vec<MaterialView>>,
}

impl MaterialContext {
    pub fn new(config: MaterialConfig, materialView: Arc<Vec<MaterialView>>) -> Self {
        Self {
            config: config,
            view: materialView,
        }

    }
}

pub fn material(
    ctx: Arc<dyn TableContext>,
    opt_ctx: Arc<MaterialContext>,
    plan: Plan,
) -> Result<Plan> {
    dbg!(&plan);
    // get material meta_data

    // diff metadata

    // rewrite

    // replace

    dbg!(&plan);
    // 匹配10次看能否找到
    // 获取注册的视图
    let curPlan = plan.clone();
    for i in 0..10 {
        //应用规则，foldLeft
        // result = rule.apply (plan)
        // equal result & plan ?
        // equal = true return
        // continue
    }
    dbg!("material start !!");
    dbg!(&opt_ctx.view);
    Ok(plan)
}

#[derive(Debug)]
pub struct MaterialView {
    plan: Plan,
    materialId: String
}

pub async fn get_register_material_view(
    ctx: Arc<dyn TableContext>) -> Result<Arc<Vec<MaterialView>>> {
    let settings = ctx.get_settings();
    // Step 1: Tokenize the SQL.
    let mut tokenizer = Tokenizer::new("select * from numbers_mt(10)").peekable();
    let mut tokens: Vec<Token> = (&mut tokenizer).collect::<Result<_>>()?;
    // Step 2: Parse the SQL.
    let backtrace = Backtrace::new();
    let (stmt, format) = parse_sql(&tokens, Dialect::MySQL, &backtrace)?;

    
    // Step 3: Bind AST with catalog, and generate a pure logical SExpr
    let metadata = Arc::new(RwLock::new(Metadata::default()));
    let name_resolution_ctx = NameResolutionContext::try_from(settings.as_ref())?;
    let binder = Binder::new(
        ctx.clone(),
        CatalogManager::instance(),
        name_resolution_ctx,
        metadata.clone(),
    );
    let materialPlan = binder.bind(&stmt).await?;

    let mut materil = MaterialView {
        plan: materialPlan,
        materialId: "view_table1".to_string(),
    };
    Ok(Arc::new(vec![materil]))
}
