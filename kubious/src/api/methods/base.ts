import { ApiContextType, ConnectionState } from "../types";

export class BaseApi {
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

    public set connection(value: ConnectionState) {
        this.state.setConnection(value);
    }
}

export type ApiConstructor<T extends BaseApi> = new (...args: any[]) => T;
export type ApiMixin<
    TBase extends BaseApi,
    TMixin extends ApiConstructor<BaseApi>
> = (Base: TBase) => TMixin;
