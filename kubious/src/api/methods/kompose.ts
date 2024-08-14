import { ApiConstructor, BaseApi } from "./base";

export function KomposeMixin<TBase extends ApiConstructor<BaseApi>>(
    Base: TBase
) {
    return class KomposeMethods extends Base {};
}
