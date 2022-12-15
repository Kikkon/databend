//  Copyright 2021 Datafuse Labs.
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

use common_base::base::tokio;
use common_catalog::plan::PushDownInfo;
use common_datavalues::prelude::*;
use common_exception::Result;
use common_sql::executor::table_read_plan::ToReadDataSourcePlan;
use databend_query::interpreters::InterpreterFactory;
use databend_query::sessions::SessionManager;
use databend_query::sessions::SessionType;
use databend_query::sessions::TableContext;
use databend_query::sql::Planner;
use databend_query::stream::ReadDataBlockStream;
use databend_query::table_functions::generate_numbers_parts;
use databend_query::table_functions::NumbersPartInfo;
use databend_query::table_functions::NumbersTable;
use futures::TryStreamExt;
use pretty_assertions::assert_eq;

use crate::tests::ConfigBuilder;
use crate::tests::TestGlobalServices;


#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_limit_push_down() -> Result<()> {
    struct Test {
        name: &'static str,
        query: &'static str,
        result: Vec<&'static str>,
    }

    let tests: Vec<Test> = vec![
        Test {
            name: "only-limit",
            query: "select * from material_cache limit 2",
            result: vec![
                "+--------+",
                "| number |",
                "+--------+",
                "| 0      |",
                "| 1      |",
                "+--------+",
            ],
        },
    ];

    let _guard = TestGlobalServices::setup(ConfigBuilder::create().build()).await?;
    for test in tests {
        let session = SessionManager::instance()
            .create_session(SessionType::Dummy)
            .await?;
        let ctx = session.create_query_context().await?;
        let mut planner = Planner::new(ctx.clone());
        let (plan, _, _) = planner.plan_sql(test.query).await?;

        let executor = InterpreterFactory::get(ctx.clone(), &plan).await?;

        let stream = executor.execute(ctx.clone()).await?;
        let result = stream.try_collect::<Vec<_>>().await?;
        let expect = test.result;
        let actual = result.as_slice();
        common_datablocks::assert_blocks_sorted_eq_with_name(test.name, expect, actual);
    }
    Ok(())
}