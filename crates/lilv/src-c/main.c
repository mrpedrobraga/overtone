#include <lilv/lilv.h>
#include <stdlib.h>

float *start(float mode)
{
    // Ok so we create a new world...
    LilvWorld *world = lilv_world_new();

    // Load some parts of the plugin definition
    // since they are defined as separate bundles (for some reason)
    //
    // This sounds neat and extensible I guess?
    lilv_world_load_specifications(world);
    lilv_world_load_plugin_classes(world);

    // Now let's load a plugin bundle :D
    LilvNode *bundle_uri = lilv_new_file_uri(world, NULL, "./plugins/testsignal.lv2");
    lilv_world_load_bundle(world, bundle_uri);

    // And summon all the plugins
    const LilvPlugins *plugin_list = lilv_world_get_all_plugins(world);

    // Load a specific plugin
    LilvNode *plugin_uri = lilv_new_uri(world, "http://gareus.org/oss/lv2/testsignal");
    const LilvPlugin *plugin = lilv_plugins_get_by_uri(plugin_list, plugin_uri);

    printf("%s\n", lilv_plugin_verify(plugin) ? "Plugin is valid" : "Oh no, plugin is not valid");
    printf("Name: %s\n", lilv_node_as_string(lilv_plugin_get_name(plugin)));

    // Let's look at the necessary features;
    LilvNodes *required_features = lilv_plugin_get_required_features(plugin);
    printf("\nRequired features\n---\n");
    LILV_FOREACH(nodes, i, required_features)
    {
        const LilvNode *feature = lilv_nodes_get(required_features, i);
        printf("Required feature: %s\n", lilv_node_as_string(feature));
    }
    printf("---\n\n");

    // Now let's create a beautiful, beautiful instance
    int sample_rate = 41000;
    LilvInstance *instance = lilv_plugin_instantiate(plugin, sample_rate, NULL);

    if (instance == NULL)
    {
        printf("Instantiation was unsuccessful... :(");
        return NULL;
    }

    uint32_t num_ports = lilv_plugin_get_num_ports(plugin);
    printf("Ports: %u\n", num_ports);
    { // Checkin the port layout.
        const LilvNode *input_uri = lilv_new_uri(world, LILV_URI_INPUT_PORT);
        const LilvNode *output_uri = lilv_new_uri(world, LILV_URI_OUTPUT_PORT);
        const LilvNode *audio_uri = lilv_new_uri(world, LILV_URI_AUDIO_PORT);
        const LilvNode *control_uri = lilv_new_uri(world, LILV_URI_CONTROL_PORT);

        for (uint32_t i = 0; i < num_ports; ++i)
        {
            const LilvPort *port = lilv_plugin_get_port_by_index(plugin, i);
            if (lilv_port_is_a(plugin, port, input_uri))
                printf("  input");
            if (lilv_port_is_a(plugin, port, output_uri))
                printf("  output");
            if (lilv_port_is_a(plugin, port, audio_uri))
                printf("  audio");
            if (lilv_port_is_a(plugin, port, control_uri))
                printf("  control");
            printf("\n");
        }
    }

    float mode_c = mode;
    float reference_c = 440.0;
    static float out[41000] = {};

    lilv_instance_connect_port(instance, 0, &mode_c);
    lilv_instance_connect_port(instance, 1, &reference_c);
    lilv_instance_connect_port(instance, 2, out);

    {
        lilv_instance_activate(instance);
        // Runs the instance!!!
        lilv_instance_run(instance, 41000);
        lilv_instance_deactivate(instance);
        lilv_instance_free(instance);
    }

    lilv_world_free(world);

    return out;
}