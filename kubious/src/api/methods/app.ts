import { ApiConstructor, BaseApi } from "./base";

export function ApplicationMixin<TBase extends ApiConstructor<BaseApi>>(
    Base: TBase
) {
    return class ApplicationMethods extends Base {};
}
