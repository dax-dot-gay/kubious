import { CommandScope } from "../types";
import { ApiConstructor, BaseApi } from "./base";

export function HelmMixin<TBase extends ApiConstructor<BaseApi>>(Base: TBase) {
    return class HelmMethods extends Base {
        public get scope() {
            return CommandScope.Helm;
        }
    };
}
