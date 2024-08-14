import { CommandScope } from "../types";
import { ApiConstructor, BaseApi } from "./base";

export function ApplicationMixin<TBase extends ApiConstructor<BaseApi>>(
    Base: TBase
) {
    return class ApplicationMethods extends Base {
        public get scope() {
            return CommandScope.Application;
        }
    };
}
