export enum CommandScope {
    Application = "app",
    Kube = "kube",
    Helm = "helm",
    Kompose = "kompose",
    Artifacts = "artifacts",
}

export type CommandSpec<
    Scope extends CommandScope,
    Command extends string,
    Args extends object = Record<string, never>
> = {
    scope: Scope;
    command: Command;
} & Args;

export type CommandResult<
    Command extends CommandSpec<any, any, any>,
    Success = any
> = {
    command: Command;
} & ({ success: true; value: Success } | { success: false; error: string });
