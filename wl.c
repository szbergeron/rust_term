#include <stdint.h>
#include <stdio.h>
#include <wayland-client.h>

static void
registry_handle_global(void *data, struct wl_registry *registry,
		uint32_t name, const char *interface, uint32_t version)
{
	printf("interface: '%s', version: %d, name: %d\n",
			interface, version, name);
}

static void
registry_handle_global_remove(void *data, struct wl_registry *registry,
		uint32_t name)
{
	// This space deliberately left blank
}

static const struct wl_registry_listener
registry_listener = {
	.global = registry_handle_global,
	.global_remove = registry_handle_global_remove,
};

static inline struct wl_registry *
iwl_display_get_registry(struct wl_display *wl_display)
{
	struct wl_proxy *registry;

	registry = wl_proxy_marshal_constructor(
            (struct wl_proxy *) wl_display,
			 WL_DISPLAY_GET_REGISTRY,
             &wl_registry_interface,
             NULL);

	return (struct wl_registry *) registry;
}

static inline int
iwl_registry_add_listener(struct wl_registry *wl_registry,
			 const struct wl_registry_listener *listener, void *data)
{
	return wl_proxy_add_listener((struct wl_proxy *) wl_registry,
				     (void (**)(void)) listener, data);
}

int
main(int argc, char *argv[])
{
	struct wl_display *display = wl_display_connect(NULL);
	struct wl_registry *registry = iwl_display_get_registry(display);
	iwl_registry_add_listener(registry, &registry_listener, NULL);

    printf("GET_REGISTRY is %d\n", WL_DISPLAY_GET_REGISTRY);
	//wl_display_roundtrip(display);
	return 0;
}
