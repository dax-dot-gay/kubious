import { CommandScope } from "../types";
import { ApiConstructor, BaseApi } from "./base";

export function KomposeMixin<TBase extends ApiConstructor<BaseApi>>(
    Base: TBase
) {
    return class KomposeMethods extends Base {
        public get scope() {
            return CommandScope.Kompose;
        }
    };
}
