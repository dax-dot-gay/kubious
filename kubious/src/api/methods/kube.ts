import { CommandScope } from "../types";
import { ApiConstructor, BaseApi } from "./base";

export function KubeMixin<TBase extends ApiConstructor<BaseApi>>(Base: TBase) {
    return class KubeMethods extends Base {
        public get scope() {
            return CommandScope.Kube;
        }
    };
}
