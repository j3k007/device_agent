from django.urls import path
from . import views

urlpatterns = [
    # Token management (admin only)
    path('tokens/', views.list_tokens, name='list_tokens'),
    path('tokens/create/', views.create_token, name='create_token'),

    # Self-registration (public)
    path('register/', views.register_device, name='register_device'),
    path('register/<str:agent_id>/status/', views.check_registration, name='check_registration'),

    # Registration management (admin only)
    path('registrations/', views.list_registrations, name='list_registrations'),
    path('registrations/<int:pk>/approve/', views.approve_registration, name='approve_registration'),
    path('registrations/<int:pk>/reject/', views.reject_registration, name='reject_registration'),
]