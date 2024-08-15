import { execute_command } from "../common";
import { ClusterConfig, CommandScope, ConnectionState } from "../types";
import { ApiConstructor, BaseApi } from "./base";

export function ApplicationMixin<TBase extends ApiConstructor<BaseApi>>(
    Base: TBase
) {
    return class ApplicationMethods extends Base {
        public async appGetCurrentConfig(): Promise<
            [string, ClusterConfig] | null
        > {
            const result = await execute_command<
                [string, ClusterConfig] | null
            >(CommandScope.Application, "get_current_config");

            if (result.success) {
                return result.value;
            } else {
                return null;
            }
        }

        public async appSetCurrentConfig(
            key: string | null
        ): Promise<ClusterConfig | null> {
            const result = await execute_command<ClusterConfig | null>(
                CommandScope.Application,
                "set_current_config",
                { key }
            );

            if (result.success) {
                if (key) {
                    this.connection = ConnectionState.active(
                        key,
                        result.value as ClusterConfig
                    );
                } else {
                    this.connection = ConnectionState.inactive();
                }
                return result.value;
            } else {
                return null;
            }
        }
    };
}
