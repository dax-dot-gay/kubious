import { camelCase, upperFirst } from "lodash";
import { CommandScope, CommandResult, CommandSpec } from "./types";
import { invoke } from "@tauri-apps/api/core";

export async function execute_command<
    Result = any,
    Scope extends CommandScope = any,
    Command extends string = any,
    Args extends object = any
>(
    scope: Scope,
    command: Command,
    options?: Args
): Promise<CommandResult<CommandSpec<Scope, Command, Args>, Result>> {
    const scopeNames = {
        [CommandScope.Application]: "Application",
        [CommandScope.Artifacts]: "Artifacts",
        [CommandScope.Helm]: "Helm",
        [CommandScope.Kompose]: "Kompose",
        [CommandScope.Kube]: "Kube",
    };

    const command_scope = scopeNames[scope];
    const command_name = upperFirst(camelCase(command));

    try {
        const result = await invoke<
            CommandResult<CommandSpec<Scope, Command, Args>, Result>
        >("execute_api_command", {
            command: {
                scope: command_scope,
                command: command_name,
                ...(options ?? {}),
            },
        });
        console.log(result);
        return result;
    } catch (e) {
        return {
            command: {
                scope: command_scope,
                command: command_name,
                ...(options ?? {}),
            } as any,
            success: false,
            error: `Command invocation failed:\n${e}`,
        };
    }
}
