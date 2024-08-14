import { execute_command } from "../common";
import {
    ApiContextType,
    CommandResult,
    CommandScope,
    CommandSpec,
} from "../types";

export class BaseApi<TScope extends CommandScope = any> {
    public constructor(public state: ApiContextType) {}

    public get connected() {
        return this.state.connection.ready;
    }

    public get clusters() {
        return this.state.clusters;
    }

    public get connection() {
        return this.state.connection;
    }

    public get current() {
        return this.state.connection.config;
    }

    // Override in children
    public get scope(): CommandScope | null {
        return null;
    }

    public async execute<
        TResult = any,
        TMethod extends string = any,
        TArgs extends object = any
    >(
        method: TMethod,
        options?: TArgs
    ): Promise<CommandResult<CommandSpec<TScope, TMethod, TArgs>, TResult>> {
        if (this.scope === null) {
            throw new Error("Cannot exec on base api");
        } else {
            return (await execute_command(this.scope, method, options)) as any;
        }
    }
}

export type ApiConstructor<T extends BaseApi> = new (...args: any[]) => T;
export type ApiMixin<
    TBase extends BaseApi,
    TMixin extends ApiConstructor<BaseApi>
> = (Base: TBase) => TMixin;
