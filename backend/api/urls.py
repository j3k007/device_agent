from django.urls import path
from . import views

urlpatterns = [
    path('heartbeat/', views.heartbeat, name='heartbeat'),
    path('health/', views.health_check, name='health_check'),
]