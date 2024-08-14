import { ApiConstructor, BaseApi } from "./base";

export function ArtifactsMixin<TBase extends ApiConstructor<BaseApi>>(
    Base: TBase
) {
    return class ArtifactsMethods extends Base {};
}
