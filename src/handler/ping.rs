// Copyright (C) 2026 The pgmoneta community
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use std::borrow::Cow;
use std::sync::Arc;

use super::PgmonetaHandler;
use crate::client::PgmonetaClient;
use rmcp::ErrorData as McpError;
use rmcp::handler::server::router::tool::{AsyncTool, ToolBase};
use rmcp::model::JsonObject;
use rmcp::schemars;

#[derive(Debug, Default, serde::Deserialize, schemars::JsonSchema)]
pub struct PingRequest {
    pub username: String,
}

/// Tool for pinging the pgmoneta management interface.
pub struct PingTool;

impl ToolBase for PingTool {
    type Parameter = PingRequest;
    type Output = String;
    type Error = McpError;

    fn name() -> Cow<'static, str> {
        "ping".into()
    }

    fn description() -> Option<Cow<'static, str>> {
        Some(
            "Ping pgmoneta via the management interface to verify connectivity and authentication."
                .into(),
        )
    }

    fn output_schema() -> Option<Arc<JsonObject>> {
        None
    }
}

impl AsyncTool<PgmonetaHandler> for PingTool {
    async fn invoke(_service: &PgmonetaHandler, request: PingRequest) -> Result<String, McpError> {
        let result: String = PgmonetaClient::request_ping(&request.username)
            .await
            .map_err(|e| {
                McpError::internal_error(format!("Failed to ping pgmoneta: {:?}", e), None)
            })?;
        PgmonetaHandler::generate_call_tool_result_string(&result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rmcp::handler::server::router::tool::ToolBase;

    #[test]
    fn test_ping_tool_metadata() {
        assert_eq!(PingTool::name(), "ping");
        assert!(PingTool::description().is_some());
    }
}
